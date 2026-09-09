import { getEnvironment } from './environment';
import {
  createCommunityAppSdkClient,
  type CommunityAppSdkClient,
} from '@sdkwork/community-pc-core/sdk';
import { createGeneratedCommunityAppSdkPort } from '@sdkwork/community-runtime';
import type { SdkworkCommunityAppSdkPort } from '@sdkwork/community-sdk-ports';
import { createClient as createFeedsOpenClient, type SdkworkFeedsClient } from '@sdkwork/feeds-sdk';
import {resolveBaseUrlWithAlignProtocol} from "@sdkwork/sdk-common";
import {AuthTokenManager} from "@sdkwork/sdk-common";

function resolvePcFeedsBaseUrl(): string {
  // Single shared base-url key; the matching API host is chosen from the
  // current page's environment+brand. The feeds open client expects a bare
  // origin, so preservePath stays off.
  return resolveBaseUrlWithAlignProtocol({ envKey: "SDKWORK_API_BASE_URL" }).url;
}

export interface SdkClients {
  appApiBaseUrl: string;
  openApiBaseUrl: string;
  communityAppSdk: CommunityAppSdkClient;
  communityAppSdkPort: SdkworkCommunityAppSdkPort;
  /** Standard feeds stream client (open surface, anonymous circle feeds). */
  feedsOpenSdkClient: SdkworkFeedsClient;
}

export function createSdkClients(tokenManager: AuthTokenManager): SdkClients {
  const env = getEnvironment();
  const communityAppSdk = createCommunityAppSdkClient({
    config: {
      appApiBaseUrl: env.appApiBaseUrl,
    },
    tokenManager,
  });

  return {
    appApiBaseUrl: env.appApiBaseUrl,
    openApiBaseUrl: env.openApiBaseUrl,
    communityAppSdk,
    communityAppSdkPort: createGeneratedCommunityAppSdkPort(communityAppSdk.client),
    feedsOpenSdkClient: createFeedsOpenClient({ baseUrl: resolvePcFeedsBaseUrl(), platform: "pc" }),
  };
}
