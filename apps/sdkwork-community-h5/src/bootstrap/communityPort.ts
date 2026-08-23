import {
  configureCommunityAuthSessionPort,
  configureCommunityFeedsPort,
  configureCommunityOrderRuntime,
  configureCommunityRuntimePort,
  type CreateCircleMembershipOrderOptions,
  type CircleMembershipOrder,
} from "@sdkwork/community-mobile-react-community";
import {
  configureOrderMobileRuntime,
  type WechatPaymentOAuthChannel,
} from "@sdkwork/order-mobile-react-orders";
import { uuid } from "@sdkwork/utils/id";
import { getRuntime } from "./runtime";

/**
 * Community H5 runtime port wiring.
 *
 * Binds the mobile React community package to the standalone application
 * runtime:
 *
 * - `configureCommunityAuthSessionPort` serves the current IAM user to the
 *   payment sheet login check.
 * - `configureCommunityRuntimePort` switches the package to the generated
 *   Community App SDK port once a session exists. Without a session the
 *   runtime stays unconfigured and community surfaces fail closed (redirect
 *   to login); there is deliberately no demo/in-memory fallback — all data
 *   and entity ids come from the backend service.
 * - `configureOrderMobileRuntime` composes the official order cashier with
 *   the order App SDK client and the IAM WeChat payment OAuth channel.
 * - `configureCommunityOrderRuntime` routes circle membership order creation
 *   through sdkwork-order (`memberships.orders.create`), so the whole
 *   purchase flow settles on the order service.
 */


function createIdempotencyKey(): string {
  return uuid();
}

function createWechatPaymentOAuthChannel(): WechatPaymentOAuthChannel {
  return {
    async fetchAuthorizeUrl(redirect: string): Promise<string> {
      const response = await getRuntime().sdkClients.iamAppSdkClient.oauth.wechatPaymentOauth.start({
        redirect,
      });
      const record = (response ?? {}) as { authorizeUrl?: unknown; authUrl?: unknown };
      const authorizeUrl = record.authorizeUrl ?? record.authUrl;
      if (typeof authorizeUrl !== "string" || authorizeUrl.trim().length === 0) {
        throw new Error("WeChat payment OAuth start did not return an authorizeUrl.");
      }
      return authorizeUrl;
    },
  };
}

let bootstrapped = false;

export function bootstrapCommunityPort(): void {
  if (bootstrapped) {
    return;
  }
  bootstrapped = true;

  const runtime = getRuntime();
  configureCommunityAuthSessionPort({
    getCurrentUser: () => runtime.getCurrentUser(),
  });

  configureOrderMobileRuntime({
    client: runtime.sdkClients.orderAppSdkClient,
    wechatPaymentOAuth: createWechatPaymentOAuthChannel(),
    paymentRegion: runtime.environment.deploymentProfile === "cloud" ? "cn" : "cn",
  });

  configureCommunityOrderRuntime({
    async createMembershipOrder(
      options: CreateCircleMembershipOrderOptions,
    ): Promise<CircleMembershipOrder> {
      const result = await runtime.sdkClients.orderAppSdkClient.memberships.orders.create(
        {
          action: "purchase",
          packageId: options.packageId,
          paymentMethod: options.paymentMethod,
          paymentProduct: "mobile_cashier_h5",
          source: options.source ?? "community-circle",
        },
        { idempotencyKey: createIdempotencyKey() },
      );
      return {
        orderId: result.orderId,
        orderNo: result.orderNo,
        amount: result.amount,
        cashierUrl: result.cashierUrl,
      };
    },
  });

  // The real generated port is installed as soon as a session exists; without
  // a session the port stays unconfigured (fail-closed, no demo fallback).
  const syncRuntimePort = (): void => {
    const user = runtime.getCurrentUser();
    if (user?.id) {
      configureCommunityRuntimePort(runtime.sdkClients.communityAppSdkPort);
      // Circle post/resource feeds read through the standard feeds stream
      // system (anonymous open surface); content writes keep the app port.
      configureCommunityFeedsPort(runtime.sdkClients.feedsOpenSdkClient);
    }
  };

  syncRuntimePort();
  void runtime
    .initialize()
    .then(syncRuntimePort)
    .catch((error: unknown) => {
      console.error("Community runtime initialization failed.", error);
    });
}
