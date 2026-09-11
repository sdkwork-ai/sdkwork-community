import type { CommunityAuthor } from './community-author';
import type { CommunityReviewState } from './community-review-state';
export interface CommunityComment {
    id: string;
    tenantId: string;
    entryId: string;
    author: CommunityAuthor;
    body: string;
    reviewState: CommunityReviewState;
    isAcceptedAnswer?: boolean;
    createdAt: string;
    updatedAt?: string;
}
//# sourceMappingURL=community-comment.d.ts.map