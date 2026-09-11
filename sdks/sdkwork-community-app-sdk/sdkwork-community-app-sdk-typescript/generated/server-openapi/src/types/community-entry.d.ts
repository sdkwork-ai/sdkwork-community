import type { CommunityAuthor } from './community-author';
import type { CommunityEntryKind } from './community-entry-kind';
import type { CommunityReviewState } from './community-review-state';
import type { CommunityStats } from './community-stats';
export interface CommunityEntry {
    id: string;
    tenantId: string;
    categoryId: string;
    categoryLabel?: string;
    author: CommunityAuthor;
    slug: string;
    kind: CommunityEntryKind;
    title: string;
    excerpt?: string;
    body?: string;
    reviewState: CommunityReviewState;
    isFeatured?: boolean;
    isPinned?: boolean;
    hasAcceptedAnswer?: boolean;
    stats: CommunityStats;
    tags?: string[];
    media?: string[];
    publishedAt?: string;
    lastActivityAt?: string;
    updatedAt?: string;
}
//# sourceMappingURL=community-entry.d.ts.map