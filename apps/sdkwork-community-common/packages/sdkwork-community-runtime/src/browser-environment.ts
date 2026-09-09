import {resolveBaseUrlWithAlignProtocol} from "@sdkwork/sdk-common";
import { trim } from "@sdkwork/utils";

const DEFAULT_DEVELOPMENT_ORIGIN = "http://127.0.0.1:18094";
const IAM_APP_SDK_FAMILY_ID = "sdkwork-iam-app-sdk";
const ORDER_APP_SDK_FAMILY_ID = "sdkwork-order-app-sdk";

export type CommunityEnvironmentName = "development" | "test" | "staging" | "production";
export type CommunityDeploymentProfile = "standalone" | "cloud";

export interface CommunityBrowserEnvironment {
  appApiBaseUrl: string;
  appKey: "sdkwork-community";
  browserOriginMode: "same-origin" | "cross-origin";
  deploymentProfile: CommunityDeploymentProfile;
  environment: CommunityEnvironmentName;
  featureFlags: {
    community: true;
  };
  iamDeploymentMode: "local" | "private" | "saas";
  iamEnvironment: "dev" | "test" | "prod";
  openApiBaseUrl: string;
  profileId: `${CommunityDeploymentProfile}.${CommunityEnvironmentName}`;
  runtimeTarget: "browser";
  sdkBaseUrls: {
    appApiBaseUrl: string;
    dependencySdkBaseUrls: {
      "sdkwork-iam-app-sdk": {
        appApiBaseUrl: string;
      };
      "sdkwork-order-app-sdk"?: {
        appApiBaseUrl?: string;
      };
    };
    openApiBaseUrl: string;
  };
}

export interface CommunityBrowserRuntimeConfigInput {
  appApiBaseUrl?: string;
  browserOriginMode?: string;
  deploymentProfile?: string;
  environment?: string;
  openApiBaseUrl?: string;
  profileId?: string;
  runtimeTarget?: string;
  sdkBaseUrls?: {
    dependencySdkBaseUrls?: Record<string, { appApiBaseUrl?: string }>;
  };
}

export interface ResolveCommunityBrowserEnvironmentInput {
  isProductionBuild: boolean;
  runtimeConfig?: CommunityBrowserRuntimeConfigInput | null;
  vite?: Record<string, string | boolean | undefined>;
}

export function resolveCommunityBrowserEnvironment(
  input: ResolveCommunityBrowserEnvironmentInput,
): CommunityBrowserEnvironment {
  const runtimeConfig = input.runtimeConfig ?? {};
  const vite = input.vite ?? {};
  const environment = resolveEnvironment(
    runtimeConfig.environment ?? stringValue(vite.VITE_SDKWORK_COMMUNITY_ENVIRONMENT),
    input.isProductionBuild,
  );
  const deploymentProfile = resolveDeploymentProfile(
    runtimeConfig.deploymentProfile
      ?? stringValue(vite.VITE_SDKWORK_COMMUNITY_DEPLOYMENT_PROFILE),
    input.isProductionBuild,
  );
  const profileId = `${deploymentProfile}.${environment}` as const;
  const configuredProfileId = optionalString(
    runtimeConfig.profileId ?? stringValue(vite.VITE_SDKWORK_COMMUNITY_PROFILE_ID),
  );
  if (configuredProfileId && configuredProfileId !== profileId) {
    throw new Error(
      `Community runtime profileId must be ${profileId}, received ${configuredProfileId}.`,
    );
  }

  const runtimeTarget = optionalString(
    runtimeConfig.runtimeTarget ?? stringValue(vite.VITE_SDKWORK_COMMUNITY_RUNTIME_TARGET),
  ) ?? (input.isProductionBuild ? "" : "browser");
  if (runtimeTarget !== "browser") {
    throw new Error(`Community browser runtimeTarget must be browser, received ${runtimeTarget || "empty"}.`);
  }

  const applicationOriginEnv = optionalString(
    stringValue(vite.VITE_SDKWORK_COMMUNITY_APPLICATION_PUBLIC_HTTP_URL),
  );
  const platformOriginEnv = optionalString(
    stringValue(vite.VITE_SDKWORK_COMMUNITY_PLATFORM_API_GATEWAY_HTTP_URL),
  );
  if (deploymentProfile === "standalone" && platformOriginEnv) {
    throw new Error(
      "Community standalone runtime must not configure a platform API gateway URL.",
    );
  }
  // Shared §6.3 defaults (ENVIRONMENT_SPEC.md): authored overrides win; when
  // absent, resolveBaseUrl derives the application edge (same-origin, page
  // host) and the cloud platform gateway (api[-<env>].<brand>; pnpm dev
  // cloud -> the local cloud-gateway dev port) from the current page host,
  // environment and deployment profile.
  const applicationOrigin = applicationOriginEnv
    ?? optionalString(resolveBaseUrlWithAlignProtocol({ mode: "standalone" }).url);
  const platformOrigin = platformOriginEnv
    ?? optionalString(resolveBaseUrlWithAlignProtocol({ mode: "cloud" }).url);
  const dependencyOrigin = deploymentProfile === "cloud"
    ? platformOrigin
    : applicationOrigin;
  const appApiBaseUrl = resolvePublicApiBaseUrl(
    runtimeConfig.appApiBaseUrl,
    applicationOrigin,
    "/app/v3/api",
    input.isProductionBuild,
    "appApiBaseUrl",
  );
  const openApiBaseUrl = resolvePublicApiBaseUrl(
    runtimeConfig.openApiBaseUrl,
    applicationOrigin,
    "/community/v3/api",
    input.isProductionBuild,
    "openApiBaseUrl",
  );
  const appbaseAppApiBaseUrl = resolvePublicApiBaseUrl(
    runtimeConfig.sdkBaseUrls?.dependencySdkBaseUrls?.[IAM_APP_SDK_FAMILY_ID]?.appApiBaseUrl,
    dependencyOrigin,
    "/app/v3/api",
    input.isProductionBuild,
    `sdkBaseUrls.dependencySdkBaseUrls.${IAM_APP_SDK_FAMILY_ID}.appApiBaseUrl`,
  );
  const browserOriginMode = resolveBrowserOriginMode(
    runtimeConfig.browserOriginMode,
    appApiBaseUrl,
    appbaseAppApiBaseUrl,
  );

  return {
    appApiBaseUrl,
    appKey: "sdkwork-community",
    browserOriginMode,
    deploymentProfile,
    environment,
    featureFlags: {
      community: true,
    },
    iamDeploymentMode: resolveIamDeploymentMode(deploymentProfile, environment),
    iamEnvironment: resolveIamEnvironment(environment),
    openApiBaseUrl,
    profileId,
    runtimeTarget: "browser",
    sdkBaseUrls: {
      appApiBaseUrl,
      dependencySdkBaseUrls: {
        [IAM_APP_SDK_FAMILY_ID]: {
          appApiBaseUrl: appbaseAppApiBaseUrl,
        },
        [ORDER_APP_SDK_FAMILY_ID]: {
          // Order resolves through the application gateway unless an explicit
          // order app API base URL is configured (mirrors the consuming-app
          // `SDKWORK_ORDER_APP_API_BASE_URL ?? IM base URL` convention).
          appApiBaseUrl: optionalString(
            runtimeConfig.sdkBaseUrls?.dependencySdkBaseUrls?.[ORDER_APP_SDK_FAMILY_ID]?.appApiBaseUrl,
          ) ?? appApiBaseUrl,
        },
      },
      openApiBaseUrl,
    },
  };
}

function resolveEnvironment(
  value: string | undefined,
  isProductionBuild: boolean,
): CommunityEnvironmentName {
  const normalized = optionalString(value) ?? (isProductionBuild ? "" : "development");
  if (
    normalized === "development"
    || normalized === "test"
    || normalized === "staging"
    || normalized === "production"
  ) {
    return normalized;
  }
  throw new Error(`Invalid Community environment: ${normalized || "empty"}.`);
}

function resolveDeploymentProfile(
  value: string | undefined,
  isProductionBuild: boolean,
): CommunityDeploymentProfile {
  const normalized = optionalString(value) ?? (isProductionBuild ? "" : "standalone");
  if (normalized === "standalone" || normalized === "cloud") {
    return normalized;
  }
  throw new Error(`Invalid Community deploymentProfile: ${normalized || "empty"}.`);
}

function resolvePublicApiBaseUrl(
  explicitValue: string | undefined,
  origin: string | undefined,
  apiPrefix: string,
  isProductionBuild: boolean,
  fieldName: string,
): string {
  const candidate = optionalString(explicitValue)
    ?? joinOriginAndPath(origin ?? (isProductionBuild ? "" : DEFAULT_DEVELOPMENT_ORIGIN), apiPrefix);
  if (!candidate) {
    throw new Error(`Community public runtime config is missing ${fieldName}.`);
  }
  return validatePublicHttpUrl(candidate, fieldName);
}

function joinOriginAndPath(origin: string, path: string): string {
  const trimmedOrigin = trim(origin);
  if (trimmedOrigin === "/") {
    return path;
  }
  const normalizedOrigin = trimmedOrigin.replace(/\/+$/u, "");
  if (!normalizedOrigin) {
    return "";
  }
  return `${normalizedOrigin}${path}`;
}

function validatePublicHttpUrl(value: string, fieldName: string): string {
  const normalized = trim(value).replace(/\/+$/u, "");
  if (normalized.startsWith("/")) {
    if (normalized.startsWith("//")) {
      throw new Error(`${fieldName} must not use a protocol-relative URL.`);
    }
    return normalized || "/";
  }

  let url: URL;
  try {
    url = new URL(normalized);
  } catch {
    throw new Error(`${fieldName} must be a root-relative or absolute HTTP(S) URL.`);
  }
  if (url.protocol !== "http:" && url.protocol !== "https:") {
    throw new Error(`${fieldName} must use HTTP or HTTPS.`);
  }
  if (url.username || url.password || url.search || url.hash) {
    throw new Error(`${fieldName} must not contain credentials, a query, or a fragment.`);
  }
  return normalized;
}

function resolveBrowserOriginMode(
  configured: string | undefined,
  appApiBaseUrl: string,
  appbaseAppApiBaseUrl: string,
): "same-origin" | "cross-origin" {
  const normalized = optionalString(configured);
  if (normalized === "same-origin" || normalized === "cross-origin") {
    return normalized;
  }
  if (normalized) {
    throw new Error(`Invalid Community browserOriginMode: ${normalized}.`);
  }
  return appApiBaseUrl.startsWith("/") && appbaseAppApiBaseUrl.startsWith("/")
    ? "same-origin"
    : "cross-origin";
}

function resolveIamDeploymentMode(
  profile: CommunityDeploymentProfile,
  environment: CommunityEnvironmentName,
): "local" | "private" | "saas" {
  if (profile === "cloud") {
    return "saas";
  }
  return environment === "development" || environment === "test" ? "local" : "private";
}

function resolveIamEnvironment(
  environment: CommunityEnvironmentName,
): "dev" | "test" | "prod" {
  if (environment === "development") {
    return "dev";
  }
  if (environment === "production") {
    return "prod";
  }
  return "test";
}

function optionalString(value: string | undefined): string | undefined {
  const normalized = trim(value ?? "");
  return normalized || undefined;
}

function stringValue(value: string | boolean | undefined): string | undefined {
  return typeof value === "string" ? value : undefined;
}
