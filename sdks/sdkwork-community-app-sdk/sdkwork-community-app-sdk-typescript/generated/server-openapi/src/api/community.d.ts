import type { ApiRequestOptions, HttpClient } from '../http/client';
import type { CommunityActivateMembershipCommand, CommunityCircleCommand, CommunityCommentCommand, CommunityEntryCommand, CommunityGroupCommand, CommunityMemberPatchCommand, CommunityMemberResponse, CommunityReactionCommand, CommunityTierCommand, SdkWorkPageData } from '../types';
export declare class CommunityCommentsApi {
    private client;
    constructor(client: HttpClient);
    /** Community comments.list */
    list(entryId: string, requestOptions?: ApiRequestOptions): Promise<SdkWorkPageData>;
    /** Community comments.create */
    create(entryId: string, body: CommunityCommentCommand, requestOptions?: ApiRequestOptions): Promise<Record<string, unknown>>;
}
export declare class CommunityReactionsApi {
    private client;
    constructor(client: HttpClient);
    /** Community reactions.set */
    create(entryId: string, body: CommunityReactionCommand, requestOptions?: ApiRequestOptions): Promise<Record<string, unknown>>;
}
export declare class CommunityEntriesPublicationReadinessApi {
    private client;
    constructor(client: HttpClient);
    /** Community entries.publicationReadiness.retrieve */
    retrieve(entryId: string, requestOptions?: ApiRequestOptions): Promise<Record<string, unknown>>;
}
export declare class CommunityEntriesRecommendationsApi {
    private client;
    constructor(client: HttpClient);
    /** Community entries.recommendations.list */
    list(entryId: string, requestOptions?: ApiRequestOptions): Promise<SdkWorkPageData>;
}
export declare class CommunityEntriesApi {
    private client;
    readonly recommendations: CommunityEntriesRecommendationsApi;
    readonly publicationReadiness: CommunityEntriesPublicationReadinessApi;
    constructor(client: HttpClient);
    /** Community entries.retrieve */
    retrieve(entryId: string, requestOptions?: ApiRequestOptions): Promise<Record<string, unknown>>;
    /** Community entries.update */
    update(entryId: string, body: CommunityEntryCommand, requestOptions?: ApiRequestOptions): Promise<Record<string, unknown>>;
    /** Community entries.delete */
    delete(entryId: string, requestOptions?: ApiRequestOptions): Promise<void>;
    /** Community entries.create */
    create(body: CommunityEntryCommand, requestOptions?: ApiRequestOptions): Promise<Record<string, unknown>>;
}
export interface CommunityFeedListParams {
    categoryId?: string;
    kind?: string;
    q?: string;
    reviewState?: string;
    tag?: string;
    page?: number;
    pageSize?: number;
}
export declare class CommunityFeedApi {
    private client;
    constructor(client: HttpClient);
    /** Community feed.list (deprecated: use the standard feeds stream system) */
    list(params?: CommunityFeedListParams, requestOptions?: ApiRequestOptions): Promise<SdkWorkPageData>;
}
export declare class CommunityGroupsApi {
    private client;
    constructor(client: HttpClient);
    /** Community groups.list */
    list(categoryId: string, requestOptions?: ApiRequestOptions): Promise<SdkWorkPageData>;
    /** Community groups.create */
    create(categoryId: string, body: CommunityGroupCommand, requestOptions?: ApiRequestOptions): Promise<Record<string, unknown>>;
    /** Community groups.update */
    update(categoryId: string, groupId: string, body: CommunityGroupCommand, requestOptions?: ApiRequestOptions): Promise<Record<string, unknown>>;
    /** Community groups.remove */
    delete(categoryId: string, groupId: string, requestOptions?: ApiRequestOptions): Promise<void>;
}
export interface CommunityTiersListParams {
    includeDisabled?: boolean;
}
export declare class CommunityTiersApi {
    private client;
    constructor(client: HttpClient);
    /** Community tiers.list */
    list(categoryId: string, params?: CommunityTiersListParams, requestOptions?: ApiRequestOptions): Promise<SdkWorkPageData>;
    /** Community tiers.create */
    create(categoryId: string, body: CommunityTierCommand, requestOptions?: ApiRequestOptions): Promise<Record<string, unknown>>;
    /** Community tiers.update */
    update(categoryId: string, tierId: string, body: CommunityTierCommand, requestOptions?: ApiRequestOptions): Promise<Record<string, unknown>>;
    /** Community tiers.delete */
    delete(categoryId: string, tierId: string, requestOptions?: ApiRequestOptions): Promise<void>;
    /** Community tiers.publish */
    publish(categoryId: string, tierId: string, requestOptions?: ApiRequestOptions): Promise<Record<string, unknown>>;
    /** Community tiers.unpublish */
    unpublish(categoryId: string, tierId: string, requestOptions?: ApiRequestOptions): Promise<Record<string, unknown>>;
}
export declare class CommunityMembersApi {
    private client;
    constructor(client: HttpClient);
    /** Community members.list */
    list(categoryId: string, requestOptions?: ApiRequestOptions): Promise<SdkWorkPageData>;
    /** Community members.current */
    retrieve(categoryId: string, requestOptions?: ApiRequestOptions): Promise<CommunityMemberResponse | null>;
    /** Community members.update */
    update(categoryId: string, memberId: string, body: CommunityMemberPatchCommand, requestOptions?: ApiRequestOptions): Promise<Record<string, unknown>>;
    /** Community members.remove */
    delete(categoryId: string, memberId: string, requestOptions?: ApiRequestOptions): Promise<void>;
    /** Community members.activate */
    activate(categoryId: string, body: CommunityActivateMembershipCommand, requestOptions?: ApiRequestOptions): Promise<Record<string, unknown>>;
}
export declare class CommunityCategoriesApi {
    private client;
    constructor(client: HttpClient);
    /** Community categories.list */
    list(requestOptions?: ApiRequestOptions): Promise<SdkWorkPageData>;
    /** Community categories.create */
    create(body: CommunityCircleCommand, requestOptions?: ApiRequestOptions): Promise<Record<string, unknown>>;
    /** Community categories.retrieve */
    retrieve(categoryId: string, requestOptions?: ApiRequestOptions): Promise<Record<string, unknown>>;
    /** Community categories.update */
    update(categoryId: string, body: CommunityCircleCommand, requestOptions?: ApiRequestOptions): Promise<Record<string, unknown>>;
    /** Community categories.delete */
    delete(categoryId: string, requestOptions?: ApiRequestOptions): Promise<void>;
    /** Community categories.join */
    join(categoryId: string, requestOptions?: ApiRequestOptions): Promise<Record<string, unknown>>;
}
export declare class CommunityApi {
    readonly categories: CommunityCategoriesApi;
    readonly members: CommunityMembersApi;
    readonly tiers: CommunityTiersApi;
    readonly groups: CommunityGroupsApi;
    readonly feed: CommunityFeedApi;
    readonly entries: CommunityEntriesApi;
    readonly reactions: CommunityReactionsApi;
    readonly comments: CommunityCommentsApi;
    constructor(client: HttpClient);
}
export declare function createCommunityApi(client: HttpClient): CommunityApi;
//# sourceMappingURL=community.d.ts.map