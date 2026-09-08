import { getEnvironment } from './environment';
import {
  createCommunityAppSdkClient,
  type CommunityAppSdkClient,
} from '@sdkwork/community-h5-core/sdk';
import { createGeneratedCommunityAppSdkPort } from '@sdkwork/community-runtime';
import type { SdkworkCommunityAppSdkPort } from '@sdkwork/community-sdk-ports';
import { createClient as createFeedsOpenClient, type SdkworkFeedsClient } from '@sdkwork/feeds-sdk';
import { createClient as createOrderAppSdkClient, type SdkworkAppClient } from '@sdkwork/order-app-sdk';
import { createClient as createIamAppSdkClient, type SdkworkAppClient as SdkworkIamAppClient } from '@sdkwork/iam-app-sdk';
import { resolveBaseUrl } from "@sdkwork/sdk-common";
import type { AuthTokenManager } from "@sdkwork/sdk-common";

export interface SdkClients {
  appApiBaseUrl: string;
  openApiBaseUrl: string;
  communityAppSdk: CommunityAppSdkClient;
  communityAppSdkPort: SdkworkCommunityAppSdkPort;
  /** Standard feeds stream client (open surface, anonymous circle feeds). */
  feedsOpenSdkClient: SdkworkFeedsClient;
  iamAppSdkClient: SdkworkIamAppClient;
  orderAppSdkClient: SdkworkAppClient;
}

function resolveCommunityH5FeedsBaseUrl(): string {
  // Single shared base-url key; the matching API host is chosen from the
  // current page's environment+brand. The feeds open client expects a bare
  // origin, so preservePath stays off.
  return resolveBaseUrl({ envKey: "SDKWORK_API_BASE_URL" }).url;
}

export function createSdkClients(tokenManager: AuthTokenManager): SdkClients {
  const env = getEnvironment();
  const communityAppSdk = createCommunityAppSdkClient({
    config: {
      appApiBaseUrl: env.appApiBaseUrl,
    },
    tokenManager,
  });

  const iamAppSdkClient = createIamAppSdkClient({
    baseUrl: env.sdkBaseUrls.dependencySdkBaseUrls['sdkwork-iam-app-sdk'].appApiBaseUrl,
    authMode: 'dual-token',
    platform: 'h5',
    tokenManager,
  });

  const orderAppSdkClient = createOrderAppSdkClient({
    baseUrl:
      env.sdkBaseUrls.dependencySdkBaseUrls['sdkwork-order-app-sdk']?.appApiBaseUrl
      ?? env.appApiBaseUrl,
    authMode: 'dual-token',
    platform: 'h5',
    tokenManager,
  });

  return {
    appApiBaseUrl: env.appApiBaseUrl,
    openApiBaseUrl: env.openApiBaseUrl,
    communityAppSdk,
    communityAppSdkPort: createGeneratedCommunityAppSdkPort(communityAppSdk.client),
    feedsOpenSdkClient: createFeedsOpenClient({
      baseUrl: resolveCommunityH5FeedsBaseUrl(),
      platform: "h5",
    }),
    iamAppSdkClient,
    orderAppSdkClient,
  };
}
