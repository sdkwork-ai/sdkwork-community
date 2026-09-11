import type { ApiRequestOptions, HttpClient } from '../http/client';
import type { CommunityCategoryCommand, CommunityCircleCommand, CommunityFeatureCommand, CommunityGroupCommand, CommunityMemberPatchCommand, CommunityModerationCommand, CommunityPinCommand, CommunityTierCommand, SdkWorkPageData } from '../types';
export interface CommunityTiersManagementListParams {
    categoryId: string;
    enabledOnly?: boolean;
}
export declare class CommunityTiersManagementApi {
    private client;
    constructor(client: HttpClient);
    /** Community tiers.management.list */
    list(params: CommunityTiersManagementListParams, requestOptions?: ApiRequestOptions): Promise<SdkWorkPageData>;
}
export interface CommunityTiersCreateParams {
    categoryId: string;
}
export interface CommunityTiersUpdateParams {
    categoryId: string;
}
export interface CommunityTiersDeleteParams {
    categoryId: string;
}
export interface CommunityTiersPublishParams {
    categoryId: string;
}
export interface CommunityTiersUnpublishParams {
    categoryId: string;
}
export declare class CommunityTiersApi {
    private client;
    readonly management: CommunityTiersManagementApi;
    constructor(client: HttpClient);
    /** Community tiers.create */
    create(body: CommunityTierCommand, params: CommunityTiersCreateParams, requestOptions?: ApiRequestOptions): Promise<Record<string, unknown>>;
    /** Community tiers.update */
    update(tierId: string, body: CommunityTierCommand, params: CommunityTiersUpdateParams, requestOptions?: ApiRequestOptions): Promise<Record<string, unknown>>;
    /** Community tiers.delete */
    delete(tierId: string, params: CommunityTiersDeleteParams, requestOptions?: ApiRequestOptions): Promise<void>;
    /** Community tiers.publish */
    publish(tierId: string, params: CommunityTiersPublishParams, requestOptions?: ApiRequestOptions): Promise<Record<string, unknown>>;
    /** Community tiers.unpublish */
    unpublish(tierId: string, params: CommunityTiersUnpublishParams, requestOptions?: ApiRequestOptions): Promise<Record<string, unknown>>;
}
export interface CommunityGroupsManagementListParams {
    categoryId: string;
}
export declare class CommunityGroupsManagementApi {
    private client;
    constructor(client: HttpClient);
    /** Community groups.management.list */
    list(params: CommunityGroupsManagementListParams, requestOptions?: ApiRequestOptions): Promise<SdkWorkPageData>;
}
export interface CommunityGroupsCreateParams {
    categoryId: string;
}
export interface CommunityGroupsUpdateParams {
    categoryId: string;
}
export interface CommunityGroupsDeleteParams {
    categoryId: string;
}
export declare class CommunityGroupsApi {
    private client;
    readonly management: CommunityGroupsManagementApi;
    constructor(client: HttpClient);
    /** Community groups.create */
    create(body: CommunityGroupCommand, params: CommunityGroupsCreateParams, requestOptions?: ApiRequestOptions): Promise<Record<string, unknown>>;
    /** Community groups.update */
    update(groupId: string, body: CommunityGroupCommand, params: CommunityGroupsUpdateParams, requestOptions?: ApiRequestOptions): Promise<Record<string, unknown>>;
    /** Community groups.delete */
    delete(groupId: string, params: CommunityGroupsDeleteParams, requestOptions?: ApiRequestOptions): Promise<void>;
}
export interface CommunityMembersManagementListParams {
    categoryId: string;
}
export declare class CommunityMembersManagementApi {
    private client;
    constructor(client: HttpClient);
    /** Community members.management.list */
    list(params: CommunityMembersManagementListParams, requestOptions?: ApiRequestOptions): Promise<SdkWorkPageData>;
}
export interface CommunityMembersUpdateParams {
    categoryId: string;
}
export interface CommunityMembersDeleteParams {
    categoryId: string;
}
export declare class CommunityMembersApi {
    private client;
    readonly management: CommunityMembersManagementApi;
    constructor(client: HttpClient);
    /** Community members.update */
    update(memberId: string, body: CommunityMemberPatchCommand, params: CommunityMembersUpdateParams, requestOptions?: ApiRequestOptions): Promise<Record<string, unknown>>;
    /** Community members.delete */
    delete(memberId: string, params: CommunityMembersDeleteParams, requestOptions?: ApiRequestOptions): Promise<void>;
}
export declare class CommunityRecommendationsApi {
    private client;
    constructor(client: HttpClient);
    /** Community recommendations.rebuild */
    rebuild(requestOptions?: ApiRequestOptions): Promise<Record<string, unknown>>;
}
export declare class CommunityModerationQueueApi {
    private client;
    constructor(client: HttpClient);
    /** Community moderation.queue.list */
    list(requestOptions?: ApiRequestOptions): Promise<SdkWorkPageData>;
}
export declare class CommunityModerationApi {
    readonly queue: CommunityModerationQueueApi;
    constructor(client: HttpClient);
}
export declare class CommunityEntriesModerationApi {
    private client;
    constructor(client: HttpClient);
    /** Community entries.moderation.create */
    create(entryId: string, body: CommunityModerationCommand, requestOptions?: ApiRequestOptions): Promise<Record<string, unknown>>;
}
export interface CommunityEntriesManagementListParams {
    categoryId?: string;
    kind?: string;
    q?: string;
    reviewState?: string;
    tag?: string;
    page?: number;
    pageSize?: number;
}
export declare class CommunityEntriesManagementApi {
    private client;
    constructor(client: HttpClient);
    /** Community entries.management.list */
    list(params?: CommunityEntriesManagementListParams, requestOptions?: ApiRequestOptions): Promise<SdkWorkPageData>;
}
export declare class CommunityEntriesApi {
    private client;
    readonly management: CommunityEntriesManagementApi;
    readonly moderation: CommunityEntriesModerationApi;
    constructor(client: HttpClient);
    /** Community entries.feature */
    feature(entryId: string, body?: CommunityFeatureCommand, requestOptions?: ApiRequestOptions): Promise<Record<string, unknown>>;
    /** Community entries.pin */
    pin(entryId: string, body?: CommunityPinCommand, requestOptions?: ApiRequestOptions): Promise<Record<string, unknown>>;
    /** Community entries.delete */
    delete(entryId: string, requestOptions?: ApiRequestOptions): Promise<void>;
}
export declare class CommunityCirclesApi {
    private client;
    constructor(client: HttpClient);
    /** Community circles.update */
    update(categoryId: string, body: CommunityCircleCommand, requestOptions?: ApiRequestOptions): Promise<Record<string, unknown>>;
}
export declare class CommunityCategoriesManagementApi {
    private client;
    constructor(client: HttpClient);
    /** Community categories.management.list */
    list(requestOptions?: ApiRequestOptions): Promise<SdkWorkPageData>;
}
export declare class CommunityCategoriesApi {
    private client;
    readonly management: CommunityCategoriesManagementApi;
    constructor(client: HttpClient);
    /** Community categories.create */
    create(body: CommunityCategoryCommand, requestOptions?: ApiRequestOptions): Promise<Record<string, unknown>>;
    /** Community categories.update */
    update(categoryId: string, body: CommunityCategoryCommand, requestOptions?: ApiRequestOptions): Promise<Record<string, unknown>>;
    /** Community categories.delete */
    delete(categoryId: string, requestOptions?: ApiRequestOptions): Promise<void>;
}
export declare class CommunityApi {
    readonly categories: CommunityCategoriesApi;
    readonly circles: CommunityCirclesApi;
    readonly entries: CommunityEntriesApi;
    readonly moderation: CommunityModerationApi;
    readonly recommendations: CommunityRecommendationsApi;
    readonly members: CommunityMembersApi;
    readonly groups: CommunityGroupsApi;
    readonly tiers: CommunityTiersApi;
    constructor(client: HttpClient);
}
export declare function createCommunityApi(client: HttpClient): CommunityApi;
//# sourceMappingURL=community.d.ts.map