import { useTranslation } from "react-i18next";
import React, { useState, useEffect } from "react";
import { useParams, useNavigate, useSearchParams } from "react-router";
import { CommunityService, isMemberLimitError, isRevenueTargetError } from "../services/CommunityService";
import { getCommunityOrderRuntime } from "../services/communityOrderRuntime";
import { getCommunityCurrentUser } from "../services/communityAuthSessionPort";
import { Community, Post, Resource, CommunityGroup, MembershipTier } from "../types";
import { cn, IconButton, showToast, Tabs } from "@sdkwork/ui-mobile-react";
import { ChevronLeft, Share2, Plus, Users, LayoutDashboard, FileText, Download, Check, Heart, MessageCircle, MessageSquare, QrCode, X, Edit2, Trash2, Newspaper, BookOpen, FolderGit, Package, Settings2, Lock } from "lucide-react";

import { PostList } from "../components/PostList";
import { ResourceList } from "../components/ResourceList";
import { GroupList } from "../components/GroupList";
import { PaymentSheet, type CirclePurchaseConfirm } from "../components/PaymentSheet";
import { SuccessModal } from "../components/SuccessModal";

import { CommunityCover } from "../components/CommunityDetail/CommunityCover";
import { CommunityLockedView } from "../components/CommunityDetail/CommunityLockedView";
import { CommentInputOverlay } from "../components/CommunityDetail/CommentInputOverlay";
import { CommunityTabsContent } from "../components/CommunityDetail/CommunityTabsContent";

const CUSTOM_TABS = ["news", "docs", "repos", "software"];

export const CommunityDetail: React.FC = () => {
  const { t } = useTranslation();
const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [searchParams] = useSearchParams();
  const [community, setCommunity] = useState<Community | null>(null);
  const [posts, setPosts] = useState<Post[]>([]);
  const [resources, setResources] = useState<Resource[]>([]);
  const [groups, setGroups] = useState<CommunityGroup[]>([]);
  const [activeTab, setActiveTab] = useState<string>(() => {
    const requested = (searchParams.get("tab") ?? "").trim();
    return ["feeds", "resources", "groups", "news", "docs", "repos", "software"].includes(requested)
      ? requested
      : "feeds";
  });
  const [isLoading, setIsLoading] = useState(true);
  const [activeCommentPostId, setActiveCommentPostId] = useState<string | null>(null);
  const [commentText, setCommentText] = useState("");
  const [tabPosts, setTabPosts] = useState<Post[]>([]);

  useEffect(() => {
    loadData();
  }, [id]);

  // Custom circle tabs (news/docs/repos/software) are tag-scoped entry
  // queries; load them when the tab becomes active instead of showing a
  // permanent empty placeholder.
  useEffect(() => {
    if (!id || !CUSTOM_TABS.includes(activeTab)) {
      setTabPosts([]);
      return undefined;
    }
    let cancelled = false;
    void CommunityService.getPostsByCommunityTag(id, activeTab)
      .then((posts) => {
        if (!cancelled) setTabPosts(posts);
      })
      .catch(() => {
        if (!cancelled) setTabPosts([]);
      });
    return () => {
      cancelled = true;
    };
  }, [activeTab, id]);

  const loadData = async () => {
    if (!id) return;
    setIsLoading(true);
    try {
      const [comm, fetchedPosts, fetchedResources, fetchedGroups] = await Promise.all([
        CommunityService.getCommunityById(id),
        CommunityService.getPostsByCommunity(id),
        CommunityService.getResourcesByCommunity(id),
        CommunityService.getGroupsByCommunity(id)
      ]);
      if (comm) setCommunity(comm);
      setPosts(fetchedPosts);
      setResources(fetchedResources);
      setGroups(fetchedGroups);
      // Preload purchasable tiers for paid circles so the join surface opens
      // with the price options already resolved (never a dead empty sheet).
      if (comm?.isPaid) {
        try {
          setTiers(await CommunityService.getMembershipTiers(id));
        } catch {
          setTiers([]);
        }
      } else {
        setTiers([]);
      }
    } catch {
      showToast(t('community.auto_fn_n5e6a908e', '获取详情失败'));
    } finally {
      setIsLoading(false);
    }
  };

  const [isPaySheetOpen, setIsPaySheetOpen] = useState(false);
  const [showSuccessModal, setShowSuccessModal] = useState(false);
  const [tiers, setTiers] = useState<MembershipTier[]>([]);

  const handleJoin = async () => {
    if (!id || !community) return;
    if (community.isPaid) {
      // Paid circles always enter the tier-selection flow; never fall through
      // to the free join path (a second trigger while the sheet is open would
      // otherwise join the circle without paying).
      if (tiers.length === 0) {
        // The preload may have failed; retry once before giving up so a
        // transient backend error cannot dead-end the purchase surface.
        try {
          const fetched = await CommunityService.getMembershipTiers(id);
          setTiers(fetched);
          if (fetched.length === 0) {
            showToast(t('community.auto_no_purchasable_tiers', '该圈子暂无可购买的会员套餐，请稍后再试或联系圈主'));
            return;
          }
        } catch {
          showToast(t('community.auto_fn_2796529c', '获取圈子配置失败'));
          return;
        }
      }
      setIsPaySheetOpen(true);
      return;
    }
    
    try {
      showToast(t('community.auto_fn_n630c7e9a', '加入中...'));
      await CommunityService.joinCommunity(id);
      
      const [fetchedResources, fetchedGroups] = await Promise.all([
        CommunityService.getResourcesByCommunity(id),
        CommunityService.getGroupsByCommunity(id)
      ]);
      setResources(fetchedResources);
      setGroups(fetchedGroups);

      setCommunity({...community, isJoined: true, memberCount: community.memberCount + 1});
      setIsPaySheetOpen(false);

      setShowSuccessModal(true);
    } catch (error) {
      showToast(
        isRevenueTargetError(error)
          ? t('community.auto_revenue_target_reached', '融资目标已达成，认购已截止')
          : isMemberLimitError(error)
            ? t('community.auto_member_limit_reached', '圈子人数已满，无法加入')
            : t('community.auto_fn_2f078e83', '操作失败'),
      );
      setIsPaySheetOpen(false);
    }
  };

  // Paid circle: create the membership order through sdkwork-order and enter
  // the cashier bridge; the bridge activates the membership and returns here.
  const handleConfirmPay = async (confirm: CirclePurchaseConfirm) => {
    const { tier, packageId, paymentMethod, isLifetime } = confirm;
    if (!id) return;
    if (!packageId) {
      showToast(t('community.auto_fn_n630c7e9a', '该会员等级尚未上架，请稍后再试'));
      return;
    }
    try {
      showToast(t('community.auto_fn_1e02c86c', '订单创建中...'));
      const order = await getCommunityOrderRuntime().createMembershipOrder({
        packageId,
        paymentMethod,
        source: 'community-circle',
      });
      setIsPaySheetOpen(false);
      navigate(`/community/${id}/cashier/${encodeURIComponent(order.orderId)}?tierId=${encodeURIComponent(tier.id)}&packageId=${encodeURIComponent(packageId)}&term=${isLifetime ? 'lifetime' : 'yearly'}`);
    } catch (error) {
      console.error('circle membership order creation failed', error);
      showToast(t('community.auto_fn_2f078e83', '下单失败，请稍后再试'));
    }
  };

  const handleLike = async (postId: string) => {
    if (!id) return;
    try {
      await CommunityService.toggleLikePost(id, postId);
      setPosts(prev => prev.map(p => {
        if (p.id === postId) {
          const isLiked = !p.isLiked;
          return {
            ...p,
            isLiked,
            likes: isLiked ? p.likes + 1 : Math.max(0, p.likes - 1)
          };
        }
        return p;
      }));
    } catch {
      showToast(t('community.auto_fn_2f078e83', '操作失败'));
    }
  };

  const handleComment = async () => {
    if (!commentText.trim() || !id || !activeCommentPostId) return;
    try {
      // The backend mints the comment id (snowflake) and returns the comment.
      const created = await CommunityService.addComment(id, activeCommentPostId, commentText);
      setPosts(prev => prev.map(p => {
        if (p.id === activeCommentPostId) {
          return { 
            ...p, 
            comments: p.comments + 1,
            commentsList: [
              ...(p.commentsList || []),
              created
            ]
          };
        }
        return p;
      }));
      setCommentText("");
      setActiveCommentPostId(null);
      showToast(t('community.auto_fn_41a16585', '评论成功'));
    } catch {
      showToast(t('community.auto_fn_41a08d0a', '评论失败'));
    }
  };

  const handleDeletePost = async (postId: string) => {
    if (!id || !window.confirm(t('community.auto_delete_post_confirm', '确定删除这条动态吗？'))) {
      return;
    }
    try {
      await CommunityService.deletePost(id, postId);
      setPosts(prev => prev.filter(post => post.id !== postId));
      setTabPosts(prev => prev.filter(post => post.id !== postId));
    } catch {
      showToast(t('community.auto_fn_2f078e83', '操作失败'));
    }
  };

  const platformNameMap: Record<string, string> = {
    wechat: t('community.platforms.wechat', '微信'),
    qq: t('community.platforms.qq', 'QQ'),
    feishu: t('community.platforms.feishu', '飞书'),
    dingtalk: t('community.platforms.dingtalk', '钉钉'),
    telegram: t('community.platforms.telegram', 'Telegram'),
    discord: t('community.platforms.discord', 'Discord'),
    whatsapp: t('community.platforms.whatsapp', 'WhatsApp'),
    other: t('community.platforms.other', '其他')
  };
  const currentUser = getCommunityCurrentUser() as { id?: string } | null;

  if (isLoading) {
    return (
      <div className="flex flex-col h-full bg-[#F2F2F7] dark:bg-black">
         <header className="h-[56px] px-4 flex items-center justify-between sticky top-0 z-10 shrink-0 pt-safe bg-transparent">
            <IconButton icon={<ChevronLeft className="w-6 h-6 text-text-main" />} className="bg-transparent w-10 h-10 -ml-2" onClick={() => navigate(-1)} />
         </header>
         <div className="flex-1 flex items-center justify-center text-text-sub">{t('community.auto_7f6f37e', '加载中...')}</div>
      </div>
    );
  }

  if (!community) {
    return (
      <div className="flex flex-col h-full bg-[#F2F2F7] dark:bg-black">
        <header className="h-[56px] px-4 flex items-center sticky top-0 z-10 pt-safe bg-bg-color">
            <IconButton icon={<ChevronLeft className="w-6 h-6 text-text-main" />} className="bg-transparent w-10 h-10 -ml-2" onClick={() => navigate(-1)} />
            <h1 className="text-[17px] font-semibold text-text-main ml-2">{t('community.auto_28f804e7', '圈子详情')}</h1>
        </header>
        <div className="flex-1 flex items-center justify-center text-text-sub">{t('community.auto_nadfe4ab', '圈子不存在')}</div>
      </div>
    );
  }

  return (
    <div className="flex flex-col h-full bg-[#F2F2F7] dark:bg-black relative">
       {/* Cover and Header */}
       <CommunityCover community={community} />

       {/* Content Area */}
       <div className="flex-1 flex flex-col bg-[#F2F2F7] dark:bg-black -mt-2 rounded-t-[24px] relative z-20 overflow-hidden shadow-[0_-4px_16px_rgba(0,0,0,0.1)]">
          {/* Join action bar if not joined */}
          {!community.isJoined && !community.isPaid && (
             <div className="px-4 py-4 bg-white dark:bg-[#1C1C1E] flex items-center justify-between shadow-[0_2px_10px_rgba(0,0,0,0.02)] z-10 shrink-0">
                <span className="text-[14px] text-text-sub pl-1">{t('community.auto_n2a576b36', '加入圈子，参与讨论并获取更多资源')}</span>
                <button 
                  className="px-5 py-2 bg-blue-500 text-white rounded-full font-medium text-[14px] shadow-md shadow-blue-500/20 active:scale-95 transition-transform"
                  onClick={handleJoin}
                >{t('community.auto_27118551', '免费加入')}</button>
             </div>
          )}

          {!community.isJoined && community.isPaid ? (
             <CommunityLockedView community={community} onJoin={handleJoin} tierCount={tiers.length} />
          ) : (
            <>
              {/* Sticky Tabs */}
              <div className="bg-white dark:bg-[#1C1C1E] flex items-center shrink-0 border-b border-black/5 dark:border-white/5">
                <Tabs
                   tabs={[
                     { id: 'feeds', name: t('community.tabs.feeds', '动态') },
                     { id: 'resources', name: t('community.tabs.resources', '资源') },
                     { id: 'groups', name: t('community.tabs.groups', '群组') },
                     { id: 'news', name: t('community.tabs.news_detail', '新闻') },
                     { id: 'docs', name: t('community.tabs.docs', '文档') },
                     { id: 'repos', name: t('community.tabs.repos', '开源') },
                     { id: 'software', name: t('community.tabs.software', '软件') }
                   ].filter(tab => {
                     const allowed = community.tabs || ['feeds', 'resources', 'groups'];
                     return allowed.includes(tab.id);
                   })}
                   activeTab={activeTab}
                   onChange={setActiveTab}
                   className="px-2"
                   itemClassName="text-[15px] px-3 py-3 font-medium text-text-sub"
                   activeItemClassName="text-[15px] font-bold text-blue-500"
                />
              </div>

              <CommunityTabsContent
                activeTab={activeTab}
                posts={posts}
                tabPosts={tabPosts}
                resources={resources}
                groups={groups}
                community={community}
                platformNameMap={platformNameMap}
                currentUserId={currentUser?.id}
                onDelete={handleDeletePost}
                onLike={handleLike}
                onCommentClick={(postId) => {
                  setActiveCommentPostId(postId);
                  setTimeout(() => {
                    document.getElementById('commentInput')?.focus();
                  }, 100);
                }}
              />
          </>
          )}
       </div>

       {/* Sub-components below Content Area */}

       {/* Floating action button for every publishable tab (if joined) */}
       {community.isJoined && ["feeds", "resources", "news", "docs", "repos", "software"].includes(activeTab) && (
         <button 
           className="absolute right-5 bottom-[4.5rem] w-14 h-14 bg-gradient-to-br from-blue-500 to-indigo-500 rounded-full shadow-lg shadow-blue-500/30 flex items-center justify-center text-white active:scale-95 transition-transform z-40"
           onClick={() => navigate(`/community/${community.id}/post?tab=${encodeURIComponent(activeTab)}`)}
         >
           <Plus className="w-7 h-7" />
         </button>
       )}
       {/* Comment Input Overlay */}
       <CommentInputOverlay
         activeCommentPostId={activeCommentPostId}
         commentText={commentText}
         setCommentText={setCommentText}
         onClose={() => setActiveCommentPostId(null)}
         onSend={handleComment}
       />

       {/* Pay Sheet Overlay */}
       {isPaySheetOpen && (
          <PaymentSheet
             communityName={community.name}
             communityCoverImage={community.coverImage}
             tiers={tiers}
             onClose={() => setIsPaySheetOpen(false)}
             onConfirm={handleConfirmPay}
          />
       )}

       {/* Success Modal */}
       {showSuccessModal && (
          <SuccessModal
             isPaid={community.isPaid}
             communityName={community.name}
             hasGroups={groups.length > 0}
             onClose={() => setShowSuccessModal(false)}
             onEnterGroups={() => {
                setShowSuccessModal(false);
                setActiveTab('groups');
             }}
             onEnterResources={() => {
                setShowSuccessModal(false);
                setActiveTab('resources');
             }}
          />
       )}

     </div>
  );
};
