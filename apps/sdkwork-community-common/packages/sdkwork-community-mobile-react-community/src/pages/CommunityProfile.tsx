import { useTranslation } from "react-i18next";
import React, { useState, useEffect } from "react";
import { useParams, useNavigate, useLocation } from "react-router";
import { CommunityService } from "../services/CommunityService";
import { Community } from "../types";
import { cn, IconButton, showToast } from "@sdkwork/ui-mobile-react";
import { ChevronLeft, ChevronRight, Camera, Image as ImageIcon, QrCode } from "lucide-react";
import { TierManagementPanel } from "../components/TierManagementPanel";
import { CommunityImage } from "../components/CommunityImage";

function formatAmount(value: number): string {
  return Number.isInteger(value) ? String(value) : value.toFixed(2);
}

export const CommunityProfile: React.FC = () => {
  const { t } = useTranslation();
const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const location = useLocation();
  
  const [community, setCommunity] = useState<Community | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    loadData();
  }, [id, location]);

  const loadData = async () => {
    if (!id) return;
    setIsLoading(true);
    try {
      const comm = await CommunityService.getCommunityById(id);
      if (comm) {
        setCommunity(comm);
      }
    } catch {
      showToast(t('community.auto_fn_2796529c', '获取圈子配置失败'));
    } finally {
      setIsLoading(false);
    }
  };

  const navigateToEditImage = (type: 'avatar' | 'coverImage') => {
  navigate(`/community/${id}/profile/image?field=${type}`);
  };

  if (isLoading && !community) {
    return (
      <div className="flex flex-col h-full bg-[#F2F2F7] dark:bg-black text-text-main">
         <header className="h-[56px] px-4 flex items-center shrink-0 pt-safe bg-bg-color">
            <IconButton icon={<ChevronLeft className="w-6 h-6" />} className="bg-transparent w-10 h-10 -ml-2" onClick={() => navigate(-1)} />
         </header>
         <div className="flex-1 flex items-center justify-center">{t('community.auto_7f6f37e', '加载中...')}</div>
      </div>
    );
  }

  if (!community) return null;

  return (
    <div className="flex flex-col h-full bg-[#F2F2F7] dark:bg-black relative text-text-main">
       <header className="h-[56px] px-4 flex items-center justify-between shrink-0 pt-safe bg-white dark:bg-[#1C1C1E] z-20 shadow-sm relative">
          <div className="absolute left-4 z-10">
            <IconButton icon={<ChevronLeft className="w-6 h-6 text-text-main" />} className="bg-transparent w-10 h-10 -ml-2" onClick={() => navigate(-1)} />
          </div>
          <h1 className="text-[17px] font-semibold flex-1 text-center">{t('community.auto_28f0bff6', '圈子信息')}</h1>
       </header>

       <div className="flex-1 overflow-y-auto pb-safe">
          
          <div className="bg-white dark:bg-[#1C1C1E] mt-4 border-y border-black/5 dark:border-white/5 pl-4">
             <div 
               className="flex items-center justify-between py-3 pr-4 border-b border-black/5 dark:border-white/5 cursor-pointer active:opacity-70 transition-opacity"
               onClick={() => navigateToEditImage('avatar')}
             >
               <span className="text-[16px]">{t('community.auto_28f1d163', '圈子头像')}</span>
               <div className="flex items-center gap-2">
                 <div className="w-12 h-12 rounded-full bg-gray-100 dark:bg-[#2C2C2E] overflow-hidden flex items-center justify-center">
                    {community.avatar ? (
                       <CommunityImage src={community.avatar} alt="Avatar" fallbackSeed={community.name} className="w-full h-full object-cover" />
                    ) : (
                       <ImageIcon className="w-5 h-5 text-text-sub opacity-50" />
                    )}
                 </div>
                 <ChevronRight className="w-5 h-5 text-text-sub opacity-50" />
               </div>
             </div>

             <div 
               className="flex items-center justify-between py-3 pr-4 cursor-pointer active:opacity-70 transition-opacity"
               onClick={() => navigateToEditImage('coverImage')}
             >
               <span className="text-[16px]">{t('community.auto_28f6b26b', '圈子背景')}</span>
               <div className="flex items-center gap-2">
                 <div className="w-20 h-12 rounded-md bg-gray-100 dark:bg-[#2C2C2E] overflow-hidden flex items-center justify-center">
                    {community.coverImage ? (
                       <img src={community.coverImage} alt="Cover" className="w-full h-full object-cover" />
                    ) : (
                       <ImageIcon className="w-5 h-5 text-text-sub opacity-50" />
                    )}
                 </div>
                 <ChevronRight className="w-5 h-5 text-text-sub opacity-50" />
               </div>
             </div>
          </div>

          <div className="bg-white dark:bg-[#1C1C1E] mt-4 border-y border-black/5 dark:border-white/5 pl-4">
             <div 
               className="flex items-center justify-between py-4 pr-4 border-b border-black/5 dark:border-white/5 cursor-pointer active:opacity-70 transition-opacity"
               onClick={() => navigate(`/community/${id}/profile/edit?field=name`)}
             >
               <span className="text-[16px] whitespace-nowrap">{t('community.auto_28f15acb', '圈子名称')}</span>
               <div className="flex items-center gap-2 flex-1 justify-end overflow-hidden pl-4">
                 <span className="text-[15px] text-text-sub truncate">{community.name}</span>
                 <ChevronRight className="w-5 h-5 text-text-sub opacity-50 shrink-0" />
               </div>
             </div>

             <div 
               className="flex items-center justify-between py-4 pr-4 border-b border-black/5 dark:border-white/5 cursor-pointer active:opacity-70 transition-opacity"
               onClick={() => navigate(`/community/${id}/profile/edit?field=description`)}
             >
               <span className="text-[16px] whitespace-nowrap">{t('community.auto_28f5f693', '圈子简介')}</span>
               <div className="flex items-center gap-2 flex-1 justify-end overflow-hidden pl-4">
                 <span className="text-[15px] text-text-sub truncate">{community.description || t('community.auto_not_set', '未设置')}</span>
                 <ChevronRight className="w-5 h-5 text-text-sub opacity-50 shrink-0" />
               </div>
             </div>

             <div 
               className="flex items-center justify-between py-4 pr-4 border-b border-black/5 dark:border-white/5 cursor-pointer active:opacity-70 transition-opacity"
               onClick={() => navigate(`/community/${id}/profile/edit?field=tags`)}
             >
               <span className="text-[16px] whitespace-nowrap">{t('community.auto_28f3c79f', '圈子标签')}</span>
               <div className="flex items-center gap-2 flex-1 justify-end overflow-hidden pl-4">
                 <span className="text-[15px] text-text-sub truncate">{community.tags.join(' ') || t('community.auto_not_set', '未设置')}</span>
                 <ChevronRight className="w-5 h-5 text-text-sub opacity-50 shrink-0" />
               </div>
             </div>

             <div 
               className="flex items-center justify-between py-4 pr-4 border-b border-black/5 dark:border-white/5 cursor-pointer active:opacity-70 transition-opacity"
               onClick={() => navigate(`/community/${id}/profile/tabs`)}
             >
               <span className="text-[16px] whitespace-nowrap">{t('community.auto_2bcd0b9b', '展示模块')}</span>
               <div className="flex items-center gap-2 flex-1 justify-end overflow-hidden pl-4">
                 <span className="text-[15px] text-text-sub truncate">{community.tabs?.length ? t('community.auto_modules_selected', '{{count}}个模块已选', { count: community.tabs.length }) : t('community.auto_modules_selected_default', '3个模块已选')}</span>
                 <ChevronRight className="w-5 h-5 text-text-sub opacity-50 shrink-0" />
               </div>
             </div>

             <div 
               className="flex items-center justify-between py-4 pr-4 border-b border-black/5 dark:border-white/5 cursor-pointer active:opacity-70 transition-opacity"
               onClick={() => navigate(`/community/${id}/profile/edit?field=memberLimit`)}
             >
               <span className="text-[16px] whitespace-nowrap">{t('community.auto_member_limit', '会员人数上限')}</span>
               <div className="flex items-center gap-2 flex-1 justify-end overflow-hidden pl-4">
                 <span className="text-[15px] text-text-sub truncate">
                   {community.memberLimit ? `${community.memberCount}/${community.memberLimit}` : t('community.auto_unlimited', '不限制')}
                 </span>
                 <ChevronRight className="w-5 h-5 text-text-sub opacity-50 shrink-0" />
               </div>
             </div>

             <div 
               className="flex items-center justify-between py-4 pr-4 border-b border-black/5 dark:border-white/5 cursor-pointer active:opacity-70 transition-opacity"
               onClick={() => navigate(`/community/${id}/profile/edit?field=revenueTarget`)}
             >
               <span className="text-[16px] whitespace-nowrap">{t('community.auto_revenue_target', '融资目标金额')}</span>
               <div className="flex items-center gap-2 flex-1 justify-end overflow-hidden pl-4">
                 <span className="text-[15px] text-text-sub truncate">
                   {community.revenueTarget
                     ? t('community.auto_funding_progress', '已募 ¥{{raised}} / 目标 ¥{{target}}', { raised: formatAmount(community.revenueRaised ?? 0), target: formatAmount(community.revenueTarget) })
                     : t('community.auto_unlimited', '不限制')}
                 </span>
                 <ChevronRight className="w-5 h-5 text-text-sub opacity-50 shrink-0" />
               </div>
             </div>

             <div 
               className="flex items-center justify-between py-4 pr-4 cursor-pointer active:opacity-70 transition-opacity"
               onClick={() => navigate(`/community/${id}/profile/qrcode`)}
             >
               <span className="text-[16px] whitespace-nowrap">{t('community.auto_nad9972f', '圈子二维码')}</span>
               <div className="flex items-center gap-2 flex-1 justify-end overflow-hidden pl-4">
                 <QrCode className="w-4 h-4 text-text-sub opacity-50 shrink-0" />
                 <ChevronRight className="w-5 h-5 text-text-sub opacity-50 shrink-0" />
               </div>
             </div>
          </div>

          <div className="bg-white dark:bg-[#1C1C1E] mt-4 border-y border-black/5 dark:border-white/5 pl-4">
             <div 
               className="flex items-center justify-between py-4 pr-4 border-b border-black/5 dark:border-white/5 cursor-pointer active:opacity-70 transition-opacity"
               onClick={() => navigate(`/community/${id}/profile/groups`)}
             >
               <span className="text-[16px]">{t('community.auto_3bf0f825', '群组管理')}</span>
               <div className="flex items-center gap-2 flex-1 justify-end">
                 <ChevronRight className="w-5 h-5 text-text-sub opacity-50 shrink-0" />
               </div>
             </div>

             <div 
               className="flex items-center justify-between py-4 pr-4 cursor-pointer active:opacity-70 transition-opacity"
               onClick={() => navigate(`/community/${id}/profile/members`)}
             >
               <span className="text-[16px]">{t('community.auto_2ddfaecd', '成员管理')}</span>
               <div className="flex items-center gap-2 flex-1 justify-end">
                 <span className="text-[15px] text-text-sub">{t('community.auto_n2bd24abe', '{{memberCount}} 人', { memberCount: community.memberCount })}</span>
                 <ChevronRight className="w-5 h-5 text-text-sub opacity-50 shrink-0" />
               </div>
             </div>
          </div>

          {community.isPaid && (
            <div className="bg-white dark:bg-[#1C1C1E] mt-4 border-y border-black/5 dark:border-white/5 px-4 py-4">
              <TierManagementPanel communityId={community.id} />
            </div>
          )}

          <div className="bg-white dark:bg-[#1C1C1E] mt-4 border-y border-black/5 dark:border-white/5 px-4 py-4">
            <button
              type="button"
              className="w-full py-3 text-[16px] font-medium text-red-500 active:opacity-60"
              onClick={() => {
                if (!window.confirm(t('community.auto_delete_circle_confirm', '确定解散该圈子吗？此操作不可恢复。'))) return;
                void CommunityService.deleteCommunity(community.id).then(() => navigate('/community'));
              }}
            >
              {t('community.auto_delete_circle', '解散圈子')}
            </button>
          </div>

       </div>
    </div>
  );
};
