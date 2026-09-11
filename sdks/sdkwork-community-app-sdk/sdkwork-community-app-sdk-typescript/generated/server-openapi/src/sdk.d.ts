import { HttpClient } from './http/client';
import type { SdkworkAppConfig } from './types/common';
import type { AuthTokenManager } from '@sdkwork/sdk-common';
import { CommunityApi } from './api/community';
export declare class SdkworkAppClient {
    private httpClient;
    readonly community: CommunityApi;
    constructor(config: SdkworkAppConfig);
    setAuthToken(token: string): this;
    setAccessToken(token: string): this;
    setTokenManager(manager: AuthTokenManager): this;
    get http(): HttpClient;
}
export declare function createClient(config: SdkworkAppConfig): SdkworkAppClient;
export default SdkworkAppClient;
//# sourceMappingURL=sdk.d.ts.map