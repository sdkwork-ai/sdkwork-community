import { useTranslation } from "react-i18next";
import React, { useState, useEffect } from "react";
import { useParams, useNavigate } from "react-router";
import { CommunityService } from "../services/CommunityService";
import { CommunityGroup } from "../types";
import { IconButton, showToast } from "@sdkwork/ui-mobile-react";
import { ChevronLeft, MessageSquare, Download, Share2 } from "lucide-react";
import {
  getCommunityMediaRuntime,
  isCommunityMediaRuntimeConfigured,
} from "../services/communityMediaRuntimePort";

export const CommunityGroupQRs: React.FC = () => {
  const { t } = useTranslation();
const { id, groupId } = useParams<{ id: string, groupId: string }>();
  const navigate = useNavigate();
  const [group, setGroup] = useState<CommunityGroup | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [displayUrls, setDisplayUrls] = useState<Record<string, string>>({});
  
  useEffect(() => {
    loadData();
  }, [id, groupId]);

  useEffect(() => {
    if (!group || !isCommunityMediaRuntimeConfigured()) {
      setDisplayUrls({});
      return;
    }
    const uris = [
      ...(group.qrCodes ?? []).map((qr) => qr.url),
      ...(group.qrCodeUrl ? [group.qrCodeUrl] : []),
    ].filter((uri) => uri.startsWith("drive://"));
    if (uris.length === 0) {
      setDisplayUrls({});
      return;
    }
    let cancelled = false;
    const port = getCommunityMediaRuntime();
    void Promise.all(uris.map(async (uri) => [uri, await port.resolveDisplayUrl?.(uri)] as const))
      .then((entries) => {
        if (cancelled) return;
        const resolved: Record<string, string> = {};
        for (const [uri, url] of entries) {
          if (url) resolved[uri] = url;
        }
        setDisplayUrls(resolved);
      })
      .catch(() => {
        if (!cancelled) setDisplayUrls({});
      });
    return () => {
      cancelled = true;
    };
  }, [group]);

  const loadData = async () => {
    if (!id || !groupId) return;
    setIsLoading(true);
    try {
      const groups = await CommunityService.getGroupsByCommunity(id);
      const found = groups.find(g => g.id === groupId);
      if (found) setGroup(found);
    } catch {
      showToast(t('community.auto_fn_6454af52', '获取群组详情失败'));
    } finally {
      setIsLoading(false);
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

  const handleSaveImage = (url: string) => {
  try {
      const resolved = displayUrls[url] ?? url;
      if (resolved.startsWith("drive://")) {
        showToast(t('community.auto_fn_25b0066f', '保存失败'));
        return;
      }
      const link = document.createElement('a');
      link.href = resolved;
      link.download = `qrcode_${Date.now()}.png`;
      link.click();
      showToast(t('community.auto_fn_n202557e9', '已保存到相册'));
    } catch {
      showToast(t('community.auto_fn_25b0066f', '保存失败'));
    }
  };

  const handleShare = async () => {
    if (!id || !group) return;
    const url = `${window.location.origin}/community/${id}`;
    try {
      if (typeof navigator.share === "function") {
        await navigator.share({ title: group.name, url });
        return;
      }
      await navigator.clipboard.writeText(url);
      showToast(t('community.auto_link_copied', '链接已复制'));
    } catch {
      showToast(t('community.auto_fn_2f078e83', '操作失败'));
    }
  };

  if (isLoading) {
    return (
      <div className="flex flex-col h-full bg-[#1C1C1E] dark:bg-black text-white">
         <header className="h-[56px] px-4 flex items-center justify-between sticky top-0 z-10 shrink-0 pt-safe bg-transparent">
            <IconButton icon={<ChevronLeft className="w-6 h-6 text-white" />} className="bg-transparent w-10 h-10 -ml-2" onClick={() => navigate(-1)} />
         </header>
         <div className="flex-1 flex items-center justify-center text-white/50">{t('community.auto_7f6f37e', '加载中...')}</div>
      </div>
    );
  }

  if (!group) {
    return (
      <div className="flex flex-col h-full bg-[#1C1C1E] dark:bg-black text-white">
        <header className="h-[56px] px-4 flex items-center sticky top-0 z-10 pt-safe bg-[#1C1C1E]">
            <IconButton icon={<ChevronLeft className="w-6 h-6 text-white" />} className="bg-transparent w-10 h-10 -ml-2" onClick={() => navigate(-1)} />
        </header>
        <div className="flex-1 flex items-center justify-center text-white/50">{t('community.auto_41804dfd', '群组不存在')}</div>
      </div>
    );
  }

  // Handle both legacy (qrCodeUrl) and new (qrCodes) data forms
  const allQrs = group.qrCodes || (group.qrCodeUrl ? [{ url: group.qrCodeUrl, description: '' }] : []);

  return (
    <div className="flex flex-col h-full bg-[#111111] dark:bg-black text-white relative">
       <header className="h-[56px] px-4 flex items-center justify-between sticky top-0 z-20 shrink-0 pt-safe bg-black/50 backdrop-blur-md">
          <IconButton icon={<ChevronLeft className="w-6 h-6 text-white" />} className="bg-transparent w-10 h-10 -ml-2" onClick={() => navigate(-1)} />
          <h1 className="text-[17px] font-semibold flex-1 text-center truncate px-2">{group.name}</h1>
          <IconButton icon={<Share2 className="w-5 h-5 text-white" />} className="bg-transparent w-10 h-10 p-0" onClick={() => void handleShare()} /></header>

       <div className="flex-1 overflow-y-auto no-scrollbar pt-6 pb-20 px-4">
          <div className="flex flex-col gap-6 items-center">
             {allQrs.map((qrItem, idx) => (
                <div key={idx} className="bg-[#222222] w-full max-w-[320px] rounded-[24px] p-6 flex flex-col items-center shadow-lg border border-white/5">
                   {qrItem.description && (
                      <div className="w-full text-center text-[15px] font-medium text-white/90 mb-4 bg-white/5 py-2 px-4 rounded-xl">
                        {qrItem.description}
                      </div>
                   )}
                   <div className="w-full aspect-square bg-white rounded-[16px] overflow-hidden p-2 relative flex items-center justify-center mx-auto max-w-[240px]">
                      <div className="absolute inset-1 border-2 border-dashed border-black/10 rounded-xl" />
                      <img src={displayUrls[qrItem.url] ?? qrItem.url} alt="QR Code" className="w-[90%] h-[90%] object-contain relative z-10" />
                   </div>
                   
                   <p className="text-[13px] text-white/50 text-center font-medium mt-6 leading-relaxed">{t('community.auto_12af9de0', '长按识别或者保存二维码')}<br />{t('community.auto_n34e77322', '打开对应 App 进行扫码')}</p>
                   
                  <button 
                     onClick={() => handleSaveImage(qrItem.url)}
                     className="flex items-center gap-1.5 mt-5 text-[14px] font-medium text-blue-400 active:opacity-70 transition-opacity px-4 py-2 bg-blue-500/10 rounded-full"
                   >
                      <Download className="w-4 h-4" />{t('community.auto_n6fcbe137', '保存到相册')}</button>
                </div>
             ))}
             
             {allQrs.length === 0 && (
               <div className="text-white/40 text-[14px]">{t('community.auto_n75772678', '暂无可用的加群二维码')}</div>
             )}
          </div>
       </div>
    </div>
  );
};
