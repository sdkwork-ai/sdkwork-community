import { createClient as createGeneratedCommunityAppClient, SdkworkAppClient, } from "../generated/server-openapi/src/index";
export { SdkworkAppClient, createGeneratedCommunityAppClient };
export * from "../generated/server-openapi/src/types";
export * from "../generated/server-openapi/src/api";
export * from "../generated/server-openapi/src/http";
export * from "../generated/server-openapi/src/auth";
export function createCommunityAppClient(config) {
    return createGeneratedCommunityAppClient(config);
}
export function createClient(config) {
    return createCommunityAppClient(config);
}
//# sourceMappingURL=index.js.map