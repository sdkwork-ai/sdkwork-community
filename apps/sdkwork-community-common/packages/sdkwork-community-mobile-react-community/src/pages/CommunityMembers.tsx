import { useTranslation } from "react-i18next";
import React, { useState, useEffect } from "react";
import { useParams, useNavigate } from "react-router";
import { CommunityService } from "../services/CommunityService";
import { CommunityMember } from "../types";
import { IconButton, showToast, Tabs } from "@sdkwork/ui-mobile-react";
import { ChevronLeft, Search, UserX } from "lucide-react";
import { MemberListItem } from "../components/CommunityMembers/MemberListItem";
import { MemberActionSheets } from "../components/CommunityMembers/MemberActionSheets";

export const CommunityMembers: React.FC = () => {
  const { t } = useTranslation();
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();

  const [members, setMembers] = useState<CommunityMember[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [activeTab, setActiveTab] = useState<'all' | 'admins' | 'blocked'>('all');
  const [searchQuery, setSearchQuery] = useState("");
  
  const [selectedMember, setSelectedMember] = useState<CommunityMember | null>(null);
  const [isActionSheetOpen, setIsActionSheetOpen] = useState(false);
  const [isBanDurationSheetOpen, setIsBanDurationSheetOpen] = useState(false);

  useEffect(() => {
    loadMembers();
  }, [id]);

  const handleInvite = async () => {
    if (!id || typeof window === "undefined") return;
    const link = `${window.location.origin}/community/${id}`;
    try {
      await navigator.clipboard.writeText(link);
      showToast(t('community.auto_fn_64f8a6d9', '已复制邀请链接'));
    } catch {
      window.prompt(t('community.auto_invite_link', '邀请链接'), link);
    }
  };

  const loadMembers = async () => {
    if (!id) return;
    setIsLoading(true);
    try {
      const data = await CommunityService.getMembersByCommunity(id);
      setMembers(data);
    } catch {
      showToast(t('community.auto_fn_n719db425', '获取成员失败'));
    } finally {
      setIsLoading(false);
    }
  };

  const handleAction = async (action: string) => {
    if (!id || !selectedMember) return;
    setIsActionSheetOpen(false);
    
    try {
      if (action === 'setAdmin') {
        await CommunityService.updateMemberRole(id, selectedMember.id, 'admin');
        showToast(t('community.auto_fn_264cce05', '已设为管理员'));
      } else if (action === 'removeAdmin') {
        await CommunityService.updateMemberRole(id, selectedMember.id, 'member');
        showToast(t('community.auto_fn_20b5c00f', '已取消管理员'));
      } else if (action === 'unban') {
        await CommunityService.updateMemberStatus(id, selectedMember.id, 'active');
        showToast(t('community.auto_fn_6ccdea32', '已解除禁言'));
      } else if (action === 'remove') {
        await CommunityService.removeMember(id, selectedMember.id);
        showToast(t('community.auto_fn_658fd6c3', '已移除成员'));
      }
      loadMembers();
    } catch {
      showToast(t('community.auto_fn_2f078e83', '操作失败'));
    }
  };

  const handleBan = async (durationText: string) => {
    if (!id || !selectedMember) return;
    setIsBanDurationSheetOpen(false);
    
    try {
      await CommunityService.updateMemberStatus(id, selectedMember.id, 'banned');
      showToast(t('community.auto_banned_toast', '已禁言 ({{duration}})', { duration: durationText }));
      loadMembers();
    } catch {
      showToast(t('community.auto_fn_2f078e83', '操作失败'));
    }
  };

  const filteredMembers = members.filter(m => {
    if (searchQuery && !m.name.toLowerCase().includes(searchQuery.toLowerCase())) {
      return false;
    }
    if (activeTab === 'admins') return m.role === 'admin' || m.role === 'owner';
    if (activeTab === 'blocked') return m.status === 'banned' || m.status === 'muted';
    return true;
  });

  if (isLoading && members.length === 0) {
    return (
      <div className="flex flex-col h-full bg-[#F2F2F7] dark:bg-black text-text-main">
         <header className="h-[56px] px-4 flex items-center justify-between shrink-0 pt-safe bg-white dark:bg-[#1C1C1E]">
            <IconButton icon={<ChevronLeft className="w-6 h-6 text-text-main" />} className="bg-transparent w-10 h-10 -ml-2" onClick={() => navigate(-1)} />
            <h1 className="text-[17px] font-semibold flex-1 text-center pr-8">{t('community.auto_2ddfaecd', '成员管理')}</h1>
         </header>
         <div className="flex-1 flex items-center justify-center text-text-sub">{t('community.auto_7f6f37e', '加载中...')}</div>
      </div>
    );
  }

  return (
    <div className="flex flex-col h-full bg-[#F2F2F7] dark:bg-black relative text-text-main">
       <header className="h-[56px] px-4 flex items-center justify-between shrink-0 pt-safe bg-white dark:bg-[#1C1C1E] z-20 relative">
          <div className="absolute left-4 z-10">
            <IconButton icon={<ChevronLeft className="w-6 h-6 text-text-main" />} className="bg-transparent w-10 h-10 -ml-2" onClick={() => navigate(-1)} />
          </div>
          <h1 className="text-[17px] font-semibold flex-1 text-center">{t('community.auto_2ddfaecd', '成员管理')}</h1>
          <div className="absolute right-4 z-10 flex items-center h-full">
            <span 
              className="text-blue-500 text-[15px] cursor-pointer font-medium active:opacity-70 flex items-center h-full px-2 -mr-2"
              onClick={() => void handleInvite()}
            >{t('community.auto_43c93dff', '邀请成员')}</span>
          </div>
       </header>

       <div className="bg-white dark:bg-[#1C1C1E] px-4 py-2 shrink-0 border-b border-black/5 dark:border-white/5 z-20 relative">
          <div className="bg-[#F2F2F7] dark:bg-[#2C2C2E] rounded-xl flex items-center px-3 h-9">
             <Search className="w-4 h-4 text-text-sub mr-2" />
             <input 
               type="text"
               value={searchQuery}
               onChange={e => setSearchQuery(e.target.value)}
               placeholder={t('community.auto_prop_n1a63f9d7', '搜索成员昵称')}
               className="bg-transparent border-none outline-none flex-1 text-[15px] text-text-main placeholder:text-text-sub"
             />
          </div>
       </div>

       <div className="bg-white dark:bg-[#1C1C1E] shrink-0 border-b border-black/5 dark:border-white/5 z-20 relative">
          <Tabs
             tabs={[
                { id: 'all', name: t('community.tabs.all_members', '全部成员') },
                { id: 'admins', name: t('community.tabs.admins', '管理员') },
                { id: 'blocked', name: t('community.tabs.blocked', '小黑屋') }
             ]}
             activeTab={activeTab}
             onChange={(tabId) => setActiveTab(tabId as any)}
             className="px-2"
             itemClassName="text-[15px] px-3 py-3 font-medium text-text-sub"
             activeItemClassName="text-blue-500 font-medium"
          />
       </div>

       <div className="flex-1 overflow-y-auto pb-safe">
          <div className="bg-white dark:bg-[#1C1C1E] mt-2 mb-8 border-y border-black/5 dark:border-white/5">
             {filteredMembers.length > 0 ? (
                filteredMembers.map((member, index) => (
                  <MemberListItem
                    key={member.id}
                    member={member}
                    isLast={index === filteredMembers.length - 1}
                    onSelect={(m) => {
                      setSelectedMember(m);
                      setIsActionSheetOpen(true);
                    }}
                  />
                ))
             ) : (
                <div className="py-20 flex flex-col items-center justify-center text-text-sub">
                   <UserX className="w-12 h-12 mb-2 opacity-20" />
                   <p className="text-[14px]">{t('community.auto_n40edb45f', '暂无相关成员')}</p>
                </div>
             )}
          </div>
       </div>

       <MemberActionSheets
          selectedMember={selectedMember}
          isActionSheetOpen={isActionSheetOpen}
          isBanDurationSheetOpen={isBanDurationSheetOpen}
          onCloseActionSheet={() => setIsActionSheetOpen(false)}
          onCloseBanSheet={() => setIsBanDurationSheetOpen(false)}
          onOpenBanSheet={() => {
            setIsActionSheetOpen(false);
            setIsBanDurationSheetOpen(true);
          }}
          onAction={handleAction}
          onBan={handleBan}
       />
    </div>
  );
};
