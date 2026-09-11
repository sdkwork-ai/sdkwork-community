import type { CommunityEntryKind } from './community-entry-kind';
export interface CommunityEntryCommand {
    categoryId: string;
    kind: CommunityEntryKind;
    title: string;
    excerpt?: string;
    body?: string;
    tags?: string[];
    media?: string[];
}
//# sourceMappingURL=community-entry-command.d.ts.map