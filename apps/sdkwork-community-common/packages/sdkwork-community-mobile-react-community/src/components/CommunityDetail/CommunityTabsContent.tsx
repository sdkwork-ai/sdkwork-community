import React from "react";
import { useTranslation } from "react-i18next";
import { PostList } from "../../components/PostList";
import { ResourceList } from "../../components/ResourceList";
import { GroupList } from "../../components/GroupList";
import { Post, Resource, CommunityGroup, Community } from "../../types";

export const CommunityTabsContent = ({
  activeTab,
  posts,
  tabPosts,
  resources,
  groups,
  community,
  platformNameMap,
  currentUserId,
  onDelete,
  onLike,
  onCommentClick,
}: {
  activeTab: string;
  posts: Post[];
  tabPosts: Post[];
  resources: Resource[];
  groups: CommunityGroup[];
  community: Community;
  platformNameMap: Record<string, string>;
  currentUserId?: string;
  onDelete?: (postId: string) => void;
  onLike: (postId: string) => void;
  onCommentClick: (postId: string) => void;
}) => {
  const { t } = useTranslation();

  // Circle-authored custom tabs (news/docs/repos/software) render the entries
  // tagged with that tab instead of a permanent empty placeholder.
  const CUSTOM_TABS = ["news", "docs", "repos", "software"];
  const isCustomTab = CUSTOM_TABS.includes(activeTab);

  return (
    <div className="flex-1 overflow-y-auto w-full">
      {activeTab === "feeds" ? (
        <PostList posts={posts} onLike={onLike} onCommentClick={onCommentClick} currentUserId={currentUserId} onDelete={onDelete} />
      ) : activeTab === "resources" ? (
        <ResourceList resources={resources} />
      ) : activeTab === "groups" ? (
        <GroupList
          groups={groups}
          communityId={community.id}
          platformNameMap={platformNameMap}
        />
      ) : isCustomTab ? (
        <PostList posts={tabPosts} onLike={onLike} onCommentClick={onCommentClick} currentUserId={currentUserId} onDelete={onDelete} />
      ) : null}
    </div>
  );
};
