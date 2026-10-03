import { useTranslation } from "react-i18next";
import React, { useState, useRef } from "react";
import { useNavigate } from "react-router";
import { CommunityService } from "../services/CommunityService";
import {
  getCommunityMediaRuntime,
  isCommunityMediaRuntimeConfigured,
} from "../services/communityMediaRuntimePort";
import { cn, IconButton, showToast } from "@sdkwork/ui-mobile-react";
import { ChevronLeft, Camera, ImagePlus } from "lucide-react";

export const CreateCommunity: React.FC = () => {
  const { t } = useTranslation();
const navigate = useNavigate();
  const fileInputRef = useRef<HTMLInputElement>(null);
  
  const [name, setName] = useState("");
  const [description, setDescription] = useState("");
  const [tags, setTags] = useState("");
  const [isPaid, setIsPaid] = useState(false);
  const [price, setPrice] = useState("");
  const [memberLimit, setMemberLimit] = useState("");
  const [revenueTarget, setRevenueTarget] = useState("");
  const [coverImage, setCoverImage] = useState<string | null>(null);
  const [coverFile, setCoverFile] = useState<File | null>(null);
  const [isSubmitting, setIsSubmitting] = useState(false);

  const handleImageChange = (e: React.ChangeEvent<HTMLInputElement>) => {
  const file = e.target.files?.[0];
    if (file) {
      // Local preview only; the file uploads through the media port on submit.
      setCoverFile(file);
      setCoverImage(URL.createObjectURL(file));
    }
  };

  const handleSubmit = async () => {
    if (!name.trim()) return showToast(t('community.auto_fn_n4fe6e34c', '请输入圈子名称'));
    if (!coverImage) return showToast(t('community.auto_fn_n44a9732a', '请上传圈子封面'));
    
    if (coverFile && !isCommunityMediaRuntimeConfigured()) {
      return showToast(t('community.auto_media_port_missing', '当前宿主未配置图片上传，无法创建带封面的圈子'));
    }

    setIsSubmitting(true);
    try {
      let storedCoverImage: string | null = null;
      if (coverFile) {
        const urls = await getCommunityMediaRuntime().uploadImages([coverFile]);
        storedCoverImage = urls[0] ?? null;
        if (!storedCoverImage) {
          throw new Error('cover upload returned no URL');
        }
      }
      const payload = {
        name,
        description,
        tags: tags.split(" ").filter(t => t.trim()),
        coverImage: storedCoverImage,
        isPaid,
        memberLimit: memberLimit.trim() ? Number(memberLimit) : undefined,
        revenueTarget: revenueTarget.trim() ? Number(revenueTarget) : undefined,
        price: isPaid ? Number(price) || 0 : undefined
      };

      await CommunityService.createCommunity(payload);
      showToast(t('community.auto_fn_26c36f0e', '创建成功'));
      navigate(-1);
    } catch {
      showToast(t('community.auto_fn_26c29693', '创建失败'));
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <div className="flex flex-col h-full bg-[#F2F2F7] dark:bg-black">
      <header className="h-[56px] px-4 flex items-center justify-between z-10 pt-safe bg-white dark:bg-[#1C1C1E] shrink-0 shadow-sm border-b border-black/5 dark:border-white/5">
         <div className="flex items-center gap-2">
            <IconButton icon={<ChevronLeft className="w-6 h-6 text-text-main" />} className="bg-transparent w-10 h-10 -ml-2" onClick={() => navigate(-1)} />
            <h1 className="text-[17px] font-semibold text-text-main">{t('community.auto_26c221c7', '创建圈子')}</h1>
         </div>
      </header>
      
      <div className="flex-1 flex flex-col pt-4 overflow-y-auto w-full gap-4 pb-safe bg-white dark:bg-[#1E1E1E]">
        
        <div className="px-4 flex flex-col items-center justify-center mb-2">
           <input 
             type="file" 
             ref={fileInputRef}
             className="hidden" 
             accept="image/*"
             onChange={handleImageChange}
           />
           <button 
              className={cn(
                "relative w-32 h-32 rounded-2xl overflow-hidden flex flex-col items-center justify-center border-2 border-dashed transition-all active:scale-[0.98]",
                coverImage ? "border-transparent" : "border-black/20 dark:border-white/20 bg-[#f8f9fa] dark:bg-[#2C2C2E]"
              )}
              onClick={() => fileInputRef.current?.click()}
           >
              {coverImage ? (
                <>
                  <img src={coverImage} alt="Cover Preview" className="w-full h-full object-cover" />
                  <div className="absolute inset-0 bg-black/40 flex items-center justify-center opacity-0 hover:opacity-100 transition-opacity">
                     <Camera className="w-8 h-8 text-white" />
                  </div>
                </>
              ) : (
                <>
                  <ImagePlus className="w-8 h-8 text-text-sub mb-2 opacity-60" />
                  <span className="text-[13px] text-text-sub font-medium">{t('community.auto_24ae4057', '上传封面')}</span>
                </>
              )}
           </button>
        </div>

        <div className="px-4">
           <label className="text-[14px] font-medium text-text-main mb-2 block">{t('community.auto_27fed6cd', '名称 *')}</label>
           <input 
             type="text"
             value={name}
             onChange={e => setName(e.target.value)}
             className="w-full bg-[#f8f9fa] dark:bg-[#2C2C2E] px-4 py-3 rounded-xl outline-none text-[15px] placeholder:text-text-sub focus:ring-1 focus:ring-blue-500 transition-shadow"
             placeholder={t('community.auto_prop_n4fe6e34c', '请输入圈子名称')}
           />
        </div>

        <div className="px-4">
           <label className="text-[14px] font-medium text-text-main mb-2 block">{t('community.auto_ca601', '描述')}</label>
           <textarea 
             value={description}
             onChange={e => setDescription(e.target.value)}
             className="w-full bg-[#f8f9fa] dark:bg-[#2C2C2E] px-4 py-3 rounded-xl outline-none text-[15px] placeholder:text-text-sub focus:ring-1 focus:ring-blue-500 transition-shadow resize-none h-24"
             placeholder={t('community.auto_prop_46d3bd84', '圈子介绍...')}
           />
        </div>

        <div className="px-4">
           <label className="text-[14px] font-medium text-text-main mb-2 block">{t('community.auto_d1457', '标签')}</label>
           <input 
             type="text"
             value={tags}
             onChange={e => setTags(e.target.value)}
             className="w-full bg-[#f8f9fa] dark:bg-[#2C2C2E] px-4 py-3 rounded-xl outline-none text-[15px] placeholder:text-text-sub focus:ring-1 focus:ring-blue-500 transition-shadow"
             placeholder={t('community.auto_prop_n78f4629', '用空格分隔，如: 科技 产品')}
           />
        </div>

        <div className="px-4 flex items-center justify-between">
           <label className="text-[14px] font-medium text-text-main block">{t('community.auto_2fbbe8da', '是否收费')}</label>
           <input 
             type="checkbox"
             checked={isPaid}
             onChange={e => setIsPaid(e.target.checked)}
             className="w-5 h-5"
           />
        </div>

        {isPaid && (
          <div className="px-4">
             <label className="text-[14px] font-medium text-text-main mb-2 block">{t('community.auto_7c6693d1', '价格 (¥)')}</label>
             <input 
               type="number"
               value={price}
               onChange={e => setPrice(e.target.value)}
               className="w-full bg-[#f8f9fa] dark:bg-[#2C2C2E] px-4 py-3 rounded-xl outline-none text-[15px] placeholder:text-text-sub focus:ring-1 focus:ring-blue-500 transition-shadow"
               placeholder={t('community.auto_prop_n12915ab8', '如: 99')}
             />
          </div>
        )}

        <div className="px-4">
           <label className="text-[14px] font-medium text-text-main mb-2 block">{t('community.auto_member_limit', '会员人数上限')}</label>
           <input 
             type="number"
             value={memberLimit}
             onChange={e => setMemberLimit(e.target.value)}
             className="w-full bg-[#f8f9fa] dark:bg-[#2C2C2E] px-4 py-3 rounded-xl outline-none text-[15px] placeholder:text-text-sub focus:ring-1 focus:ring-blue-500 transition-shadow"
             placeholder={t('community.auto_unlimited_hint', '不填表示不限制')}
           />
           <p className="text-[12px] text-text-sub mt-1">{t('community.auto_member_limit_hint', '达到上限后新成员无法加入')}</p>
        </div>

        <div className="px-4">
           <label className="text-[14px] font-medium text-text-main mb-2 block">{t('community.auto_revenue_target_hint', '融资目标金额（累计支付上限）')}</label>
           <input 
             type="number"
             value={revenueTarget}
             onChange={e => setRevenueTarget(e.target.value)}
             className="w-full bg-[#f8f9fa] dark:bg-[#2C2C2E] px-4 py-3 rounded-xl outline-none text-[15px] placeholder:text-text-sub focus:ring-1 focus:ring-blue-500 transition-shadow"
             placeholder={t('community.auto_revenue_placeholder', '不填表示不限制，如天使轮 500000')}
           />
           <p className="text-[12px] text-text-sub mt-1">{t('community.auto_revenue_hint', '会员购买金额累计达到目标后停止销售')}</p>
        </div>

        <div className="h-24"></div> {/* Spacer for fixed bottom */}
      </div>

      <div className="fixed bottom-0 left-0 right-0 p-4 bg-white dark:bg-[#1E1E1E] border-t border-black/5 dark:border-white/5 pb-safe z-10">
        <button 
          className={cn(
            "w-full h-12 rounded-full font-bold text-[16px] text-white flex items-center justify-center active:scale-[0.98] transition-all",
            isSubmitting || !name.trim() || !coverImage ? "bg-blue-300 pointer-events-none" : "bg-blue-500 shadow-md shadow-blue-500/20"
          )}
          onClick={handleSubmit}
        >{isSubmitting ? t('community.auto_creating', '创建中...') : t('community.auto_create_now', '立即创建')}</button>
      </div>
    </div>
  );
};
