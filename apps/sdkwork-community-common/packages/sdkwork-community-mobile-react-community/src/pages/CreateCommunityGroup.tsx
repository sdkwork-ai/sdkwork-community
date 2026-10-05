import { useTranslation } from "react-i18next";
import React, { useState, useEffect } from "react";
import { useParams, useNavigate } from "react-router";
import { CommunityService } from "../services/CommunityService";
import { CommunityGroup, QRCodeItem } from "../types";
import {
  getCommunityMediaRuntime,
  isCommunityMediaRuntimeConfigured,
} from "../services/communityMediaRuntimePort";
import { cn, IconButton, showToast } from "@sdkwork/ui-mobile-react";
import { ChevronLeft, Plus, X, UploadCloud, MessageSquare } from "lucide-react";

export const CreateCommunityGroup: React.FC = () => {
  const { t } = useTranslation();
const { id, groupId } = useParams<{ id: string, groupId?: string }>();
  const navigate = useNavigate();
  const [name, setName] = useState("");
  const [platform, setPlatform] = useState<CommunityGroup['platform']>('wechat');
  const [description, setDescription] = useState("");
  const [qrCodes, setQrCodes] = useState<QRCodeItem[]>([]);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [isLoading, setIsLoading] = useState(!!groupId);
  // Group QR codes upload through the host media runtime port (drive-backed,
  // the same channel posts use) instead of being read as base64 data URLs.
  // Hosts without the port hide the pick affordances; stored drive:// codes
  // resolve their display through the port's bounded preview reader.
  const mediaUploadEnabled = isCommunityMediaRuntimeConfigured();
  const [qrDisplayUrls, setQrDisplayUrls] = useState<Record<string, string>>({});

  useEffect(() => {
    const driveUris = Array.from(new Set(qrCodes.map((qr) => qr.url).filter((url) => url.startsWith('drive://'))));
    if (!mediaUploadEnabled || driveUris.length === 0) {
      setQrDisplayUrls({});
      return;
    }
    let cancelled = false;
    const mediaRuntime = getCommunityMediaRuntime();
    void Promise.all(driveUris.map(async (uri) => [uri, await mediaRuntime.resolveDisplayUrl?.(uri)] as const))
      .then((entries) => {
        if (cancelled) return;
        const resolved: Record<string, string> = {};
        for (const [uri, url] of entries) {
          if (url) resolved[uri] = url;
        }
        setQrDisplayUrls(resolved);
      })
      .catch(() => {
        if (!cancelled) setQrDisplayUrls({});
      });
    return () => {
      cancelled = true;
    };
  }, [qrCodes, mediaUploadEnabled]);

  const isEditMode = !!groupId;

  useEffect(() => {
    if (groupId && id) {
      loadGroup();
    }
  }, [id, groupId]);

  const loadGroup = async () => {
    try {
      const groups = await CommunityService.getGroupsByCommunity(id!);
      const group = groups.find(g => g.id === groupId);
      if (group) {
        setName(group.name);
        setPlatform(group.platform);
        setDescription(group.description || "");
        setQrCodes(group.qrCodes || (group.qrCodeUrl ? [{ url: group.qrCodeUrl, description: '' }] : []));
      }
    } catch {
      showToast(t('community.auto_fn_n638c6acd', '获取群组失败'));
    } finally {
      setIsLoading(false);
    }
  };

  const handleAddQr = () => {
    if (!isCommunityMediaRuntimeConfigured()) {
      // No host media runtime: there is deliberately no local data-URL
      // fallback — persisting a base64 payload would be a fake upload
      // (`DRIVE_SPEC.md` section 18).
      showToast(t('community.auto_media_unavailable', '媒体上传不可用：需要宿主提供存储能力'));
      return;
    }
  const input = document.createElement('input');
    input.type = 'file';
    input.accept = 'image/*';
    input.onchange = (e) => {
      const file = (e.target as HTMLInputElement).files?.[0];
      if (file) {
        void getCommunityMediaRuntime()
          .uploadImages([file])
          .then(([url]) => {
            if (url) {
              setQrCodes(prev => [...prev, { url, description: '' }]);
            }
          })
          .catch(() => {
            showToast(t('community.auto_media_upload_failed', '二维码上传失败，请重试'));
          });
      }
    };
    input.click();
  };

  const handleUpdateQrDescription = (index: number, value: string) => {
  const updated = [...qrCodes];
    updated[index].description = value;
    setQrCodes(updated);
  };

  const handleRemoveQr = (index: number) => {
  setQrCodes(qrCodes.filter((_, i) => i !== index));
  };

  const handleSubmit = async () => {
    if (!id) return;
    if (!name.trim()) return showToast(t('community.auto_fn_n3cec0af4', '请输入群组名称'));
    if (qrCodes.length === 0) return showToast(t('community.auto_fn_525f090e', '请至少上传一张二维码'));
    
    setIsSubmitting(true);
    try {
      const payload = {
        name,
        platform,
        description,
        memberCount: isEditMode ? undefined : 0,
        qrCodes
      };

      if (isEditMode && groupId) {
        await CommunityService.updateGroup(id, groupId, payload);
        showToast(t('community.auto_fn_n10752d86', '群组修改成功'));
      } else {
        await CommunityService.createGroup(id, payload as any);
        showToast(t('community.auto_fn_nf8f6d52', '群组创建成功'));
      }
      navigate(-1);
    } catch {
      showToast(isEditMode ? t('community.auto_fn_update_failed', '修改失败') : t('community.auto_fn_26c29693', '创建失败'));
    } finally {
      setIsSubmitting(false);
    }
  };

  if (isLoading) {
    return (
      <div className="flex flex-col h-full bg-[#F2F2F7] dark:bg-black">
        <header className="h-[56px] px-4 flex items-center sticky top-0 z-10 pt-safe bg-bg-color shrink-0">
            <IconButton icon={<ChevronLeft className="w-6 h-6 text-text-main" />} className="bg-transparent w-10 h-10 -ml-2" onClick={() => navigate(-1)} />
        </header>
        <div className="flex-1 flex items-center justify-center text-text-sub">{t('community.auto_7f6f37e', '加载中...')}</div>
      </div>
    );
  }

  return (
    <div className="flex flex-col h-full bg-[#F2F2F7] dark:bg-black">
      <header className="h-[56px] px-4 flex items-center justify-between sticky top-0 z-10 pt-safe bg-bg-color shrink-0 shadow-sm border-b border-black/5 dark:border-white/5">
         <div className="flex items-center gap-2">
            <IconButton icon={<ChevronLeft className="w-6 h-6 text-text-main" />} className="bg-transparent w-10 h-10 -ml-2" onClick={() => navigate(-1)} />
            <h1 className="text-[17px] font-semibold text-text-main">{isEditMode ? t('community.auto_3bf0041b', '编辑群组') : t('community.auto_add_group', '添加群组')}</h1>
         </div>
      </header>
      
      <div className="flex-1 flex flex-col pt-4 overflow-y-auto w-full gap-4 pb-safe bg-white dark:bg-[#1E1E1E]">
        
        <div className="px-4">
           <label className="text-[14px] font-medium text-text-main mb-2 block">{t('community.auto_ne540593', '群组名称 *')}</label>
           <input 
             type="text"
             value={name}
             onChange={e => setName(e.target.value)}
             className="w-full bg-[#f8f9fa] dark:bg-[#2C2C2E] px-4 py-3 rounded-2xl outline-none text-[15px] placeholder:text-text-sub focus:ring-1 focus:ring-blue-500 transition-shadow"
             placeholder={t('community.auto_prop_80284a', '如: AI 开发者微信1群')}
           />
        </div>

        <div className="px-4">
           <label className="text-[14px] font-medium text-text-main mb-2 block">{t('community.auto_2c2a44a7', '平台 *')}</label>
           <select 
             value={platform}
             onChange={e => setPlatform(e.target.value as any)}
             className="w-full bg-[#f8f9fa] dark:bg-[#2C2C2E] px-4 py-3 rounded-2xl outline-none text-[15px] focus:ring-1 focus:ring-blue-500 transition-shadow appearance-none"
           >
             <option value="wechat">{t('community.auto_be5f3', '微信')}</option>
             <option value="qq">QQ</option>
             <option value="telegram">Telegram</option>
             <option value="discord">Discord</option>
             <option value="feishu">{t('community.auto_12d148', '飞书')}</option>
             <option value="dingtalk">{t('community.auto_129120', '钉钉')}</option>
             <option value="whatsapp">WhatsApp</option>
             <option value="other">{t('community.auto_a2c20', '其他')}</option>
           </select>
        </div>

        <div className="px-4">
           <label className="text-[14px] font-medium text-text-main mb-2 block">{t('community.auto_719c787e', '描述 (选填)')}</label>
           <textarea 
             value={description}
             onChange={e => setDescription(e.target.value)}
             className="w-full bg-[#f8f9fa] dark:bg-[#2C2C2E] px-4 py-3 rounded-2xl outline-none text-[15px] placeholder:text-text-sub focus:ring-1 focus:ring-blue-500 transition-shadow resize-none h-24"
             placeholder={t('community.auto_prop_484b22b', '群组规则或介绍...')}
           />
        </div>

        <div className="px-4 pt-2">
           <label className="text-[14px] font-medium text-text-main mb-3 flex items-center justify-between">
              <span>{t('community.auto_14ffcc7b', '二维码 (可传多张) *')}</span>
              <button className="text-blue-500 text-[13px] flex items-center gap-1 active:opacity-70 transition-opacity" onClick={handleAddQr}>
                 <Plus className="w-4 h-4"/>{t('community.auto_29887662', '增加一张')}</button>
           </label>
           
           <div className="flex flex-col gap-4">
              {qrCodes.map((qr, index) => (
                 <div key={index} className="flex gap-3 bg-[#f8f9fa] dark:bg-[#2C2C2E] p-3 rounded-2xl relative border border-transparent focus-within:border-blue-500 transition-colors">
                    <div className="w-[100px] h-[100px] relative shrink-0 rounded-xl overflow-hidden bg-black/5 dark:bg-white/5 border border-black/5 dark:border-white/5">
                       <img src={qrDisplayUrls[qr.url] ?? qr.url} alt="" className="w-full h-full object-cover" />
                       <button 
                         onClick={() => handleRemoveQr(index)}
                         className="absolute right-1 top-1 w-6 h-6 bg-black/40 text-white rounded-full flex items-center justify-center backdrop-blur-md"
                       >
                         <X className="w-4 h-4" />
                       </button>
                    </div>
                    <div className="flex-1 flex flex-col justify-between">
                       <textarea
                         value={qr.description}
                         onChange={e => handleUpdateQrDescription(index, e.target.value)}
                         placeholder={t('community.auto_prop_n51fc90ee', '添加提示文案，例如: 群1已满，请加群2...')}
                         className="w-full h-full bg-transparent outline-none text-[14px] resize-none text-text-main placeholder:text-text-sub inline-block pt-1"
                       />
                    </div>
                 </div>
              ))}
              
              {qrCodes.length === 0 && (
                <div 
                  className="h-28 border-2 border-dashed border-black/20 dark:border-white/20 rounded-2xl flex flex-col items-center justify-center text-text-sub cursor-pointer active:bg-black/5 transition-colors gap-2"
                  onClick={handleAddQr}
                >
                  <UploadCloud className="w-8 h-8 opacity-50" />
                  <span className="text-[14px] font-medium opacity-80">{t('community.auto_n54e7f17b', '点击上传第一张二维码')}</span>
                </div>
              )}
           </div>
        </div>

        <div className="px-4 py-8 mt-auto">
          <button 
            className={cn(
              "w-full h-12 rounded-full font-bold text-[16px] text-white flex items-center justify-center active:scale-[0.98] transition-all",
              isSubmitting || qrCodes.length === 0 || !name.trim() ? "bg-blue-300 pointer-events-none" : "bg-blue-500 shadow-md shadow-blue-500/20"
            )}
            onClick={handleSubmit}
          >{isSubmitting ? t('community.auto_saving', '保存中...') : (isEditMode ? t('community.auto_save_changes', '保存修改') : t('community.auto_create_now', '立即创建'))}</button>
        </div>

      </div>
    </div>
  );
};
