import { useTranslation } from "react-i18next";
import React, { useState, useEffect, useRef } from "react";
import { useParams, useNavigate } from "react-router";
import { CommunityService } from "../services/CommunityService";
import { Community } from "../types";
import { cn, IconButton, showToast } from "@sdkwork/ui-mobile-react";
import { ChevronLeft, MoreVertical, Download, Share2, MessageCircle } from "lucide-react";
import { QRCodeCanvas } from "qrcode.react";
import { toPng } from "html-to-image";
import { CommunityImage } from "../components/CommunityImage";

export const CommunityQRCode: React.FC = () => {
  const { t } = useTranslation();
const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();

  const [community, setCommunity] = useState<Community | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [isActionSheetOpen, setIsActionSheetOpen] = useState(false);
  const cardRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (id) {
      CommunityService.getCommunityById(id).then(c => {
        if (c) setCommunity(c);
        setIsLoading(false);
      });
    }
  }, [id]);

  const saveToAlbum = async () => {
    if (!cardRef.current) return;
    try {
      showToast(t('community.auto_fn_n58b0a331', '正在生成图片...'));
      const dataUrl = await toPng(cardRef.current, {
        cacheBust: true,
        backgroundColor: '#FFFFFF',
        pixelRatio: 2,
        style: { transform: 'scale(1)', margin: '0' }
      });
      const link = document.createElement('a');
      link.download = `${community?.name}-${t('community.auto_28f15322', '圈子名片')}.png`;
      link.href = dataUrl;
      link.click();
      showToast(t('community.auto_fn_n202557e9', '已保存到相册'));
      setIsActionSheetOpen(false);
    } catch (err) {
      console.error(err);
      showToast(t('community.auto_fn_25b0066f', '保存失败'));
    }
  };

  const handleShare = async (type: string) => {
    if (!community) return;
    const url = `${window.location.origin}/community/${community.id}`;
    try {
      if (typeof navigator.share === "function") {
        await navigator.share({
          title: community.name,
          text: t('community.auto_share_text', '加入圈子：{{name}}', { name: community.name }),
          url,
        });
      } else {
        await navigator.clipboard.writeText(url);
        showToast(t('community.auto_link_copied', '链接已复制'));
      }
    } catch {
      // The platform share sheet can be dismissed by the user; nothing to do.
    } finally {
      setIsActionSheetOpen(false);
    }
  };

  if (isLoading || !community) {
    return (
      <div className="flex flex-col h-full bg-[#1C1C1E] text-white">
         <header className="h-[56px] px-4 flex items-center justify-between shrink-0 pt-safe z-20">
            <IconButton icon={<ChevronLeft className="w-6 h-6 text-white" />} className="bg-transparent w-10 h-10 -ml-2" onClick={() => navigate(-1)} />
         </header>
      </div>
    );
  }

  const shareUrl = window.location.origin + `/community/${id}`;

  return (
    <div className="flex flex-col h-full bg-[#EAEAEA] dark:bg-[#000000] relative text-text-main">
       <header className="h-[56px] px-4 flex items-center justify-between shrink-0 pt-safe z-20 relative">
          <div className="absolute left-4 z-10 flex items-center h-full">
             <IconButton icon={<ChevronLeft className="w-6 h-6 text-black dark:text-white" />} className="bg-transparent w-10 h-10 -ml-2" onClick={() => navigate(-1)} />
          </div>
          <h1 className="text-[17px] font-semibold flex-1 text-center text-black dark:text-white">{t('community.auto_28f15322', '圈子名片')}</h1>
          <div className="absolute right-4 z-10 flex items-center h-full">
             <IconButton 
                icon={<MoreVertical className="w-6 h-6 text-black dark:text-white" />} 
                className="bg-transparent w-10 h-10 -mr-2" 
                onClick={() => setIsActionSheetOpen(true)} 
             />
          </div>
       </header>

       <div className="flex-1 overflow-y-auto pb-safe flex flex-col items-center pt-[10vh] px-6">
          <div 
             ref={cardRef}
             className="w-full max-w-[340px] bg-white rounded-xl shadow-lg flex flex-col p-6 items-center"
          >
             <div className="flex items-center w-full gap-4 mb-8">
                <div className="w-14 h-14 rounded-full overflow-hidden bg-gray-100 shrink-0">
                   <CommunityImage src={community.avatar} alt="avatar" fallbackSeed={community.name} className="w-full h-full object-cover" />
                </div>
                <div className="flex flex-col flex-1 overflow-hidden">
                   <span className="text-[17px] font-semibold text-black truncate">{community.name}</span>
                   <span className="text-[13px] text-gray-500 mt-1 truncate">{t('community.auto_n6376b5fd', '扫一扫二维码，加入我们的圈子')}</span>
                </div>
             </div>

             <div className="w-full aspect-square max-w-[240px] bg-white flex items-center justify-center p-2 mb-8">
                <QRCodeCanvas 
                   value={shareUrl} 
                   size={220}
                   level="H"
                   imageSettings={community.avatar ? {
                      src: community.avatar,
                      x: undefined,
                      y: undefined,
                      height: 48,
                      width: 48,
                      excavate: true,
                   } : undefined}
                />
             </div>

             <div className="text-[13px] text-gray-400">{t('community.auto_n78043323', '邀请你加入【{{name}}】', { name: community?.name ?? '' })}</div>
          </div>
       </div>

       {isActionSheetOpen && (
          <div className="fixed inset-0 z-50 flex flex-col justify-end pointer-events-auto">
             <div 
               className="absolute inset-0 bg-black/40 transition-opacity"
               onClick={() => setIsActionSheetOpen(false)}
             />
             <div className="bg-[#F2F2F7] dark:bg-[#1C1C1E] rounded-t-2xl w-full max-w-md mx-auto relative z-10 overflow-hidden pb-safe animate-in slide-in-from-bottom duration-300">
                <div className="flex flex-col">
                   <button 
                      onClick={() => handleShare(t('community.auto_wechat_friend', '微信好友'))}
                      className="bg-white dark:bg-[#2C2C2E] py-4 text-[16px] text-text-main border-b border-black/5 dark:border-white/5 active:bg-black/5 dark:active:bg-white/5 transition-colors flex items-center justify-center gap-2"
                   >
                      <MessageCircle className="w-5 h-5 text-green-500" />{t('community.auto_n470c7ad6', '发送给微信好友')}</button>
                   <button 
                      onClick={() => handleShare(t('community.auto_moments', '朋友圈'))}
                      className="bg-white dark:bg-[#2C2C2E] py-4 text-[16px] text-text-main border-b border-black/5 dark:border-white/5 active:bg-black/5 dark:active:bg-white/5 transition-colors flex items-center justify-center gap-2"
                   >
                      <Share2 className="w-5 h-5 text-blue-500" />{t('community.auto_731c8f9d', '分享到朋友圈')}</button>
                   <button 
                      onClick={saveToAlbum}
                      className="bg-white dark:bg-[#2C2C2E] py-4 text-[16px] text-text-main active:bg-black/5 dark:active:bg-white/5 transition-colors flex items-center justify-center gap-2"
                   >
                      <Download className="w-5 h-5 text-text-sub" />{t('community.auto_4aa19c18', '保存到手机相册')}</button>
                </div>
                
                <div className="mt-2">
                   <button 
                      onClick={() => setIsActionSheetOpen(false)}
                      className="w-full bg-white dark:bg-[#2C2C2E] py-4 text-[16px] font-medium text-text-main active:bg-black/5 dark:active:bg-white/5 transition-colors"
                   >{t('community.auto_a9472', '取消')}</button>
                </div>
             </div>
          </div>
       )}
    </div>
  );
};
