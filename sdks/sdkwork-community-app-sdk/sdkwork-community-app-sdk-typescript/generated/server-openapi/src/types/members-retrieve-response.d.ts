import type { CommunityMemberResponse } from './community-member-response';
export interface MembersRetrieveResponse {
    code: 0;
    data: unknown & {
        item: CommunityMemberResponse | null;
    };
    /** Server-owned request correlation id. */
    traceId: string;
}
//# sourceMappingURL=members-retrieve-response.d.ts.map