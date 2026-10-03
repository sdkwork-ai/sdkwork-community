import { useTranslation } from "react-i18next";
import React, { useState, useEffect, useRef, useCallback } from "react";
import { useParams, useNavigate, useSearchParams } from "react-router";
import { CommunityService } from "../services/CommunityService";
import {
  getCommunityMediaRuntime,
  isCommunityMediaRuntimeConfigured,
} from "../services/communityMediaRuntimePort";
import { Community } from "../types";
import { cn, IconButton, showToast } from "@sdkwork/ui-mobile-react";
import { ChevronLeft, Camera, Image as ImageIcon } from "lucide-react";
import Cropper from 'react-easy-crop';

const createImage = (url: string) =>
  new Promise<HTMLImageElement>((resolve, reject) => {
    const image = new Image()
    image.addEventListener('load', () => resolve(image))
    image.addEventListener('error', (error) => reject(error))
    image.setAttribute('crossOrigin', 'anonymous')
    image.src = url
  })

async function getCroppedImg(
  imageSrc: string,
  pixelCrop: { x: number; y: number; width: number; height: number }
): Promise<{ previewUrl: string; file: File }> {
  const image = await createImage(imageSrc)
  const canvas = document.createElement('canvas')
  const ctx = canvas.getContext('2d')

  if (!ctx) return ''

  canvas.width = image.width
  canvas.height = image.height

  ctx.drawImage(image, 0, 0)

  const croppedCanvas = document.createElement('canvas')
  const croppedCtx = croppedCanvas.getContext('2d')
  if (!croppedCtx) return ''
  
  croppedCanvas.width = pixelCrop.width
  croppedCanvas.height = pixelCrop.height

  croppedCtx.drawImage(
    canvas,
    pixelCrop.x,
    pixelCrop.y,
    pixelCrop.width,
    pixelCrop.height,
    0,
    0,
    pixelCrop.width,
    pixelCrop.height
  )

  return new Promise((resolve, reject) => {
    croppedCanvas.toBlob((blob) => {
      if (blob) {
        // Local preview plus the bytes the media port uploads on save.
        resolve({
          previewUrl: URL.createObjectURL(blob),
          file: new File([blob], 'crop.jpg', { type: 'image/jpeg' }),
        })
      } else {
        reject(new Error('Canvas is empty'))
      }
    }, 'image/jpeg', 0.9)
  })
}

export const CommunityEditImage: React.FC = () => {
  const { t } = useTranslation();
const { id } = useParams<{ id: string }>();
  const [searchParams] = useSearchParams();
  const field = searchParams.get('field') as 'avatar' | 'coverImage' || 'avatar';
  const navigate = useNavigate();

  const [community, setCommunity] = useState<Community | null>(null);
  const [imageUrl, setImageUrl] = useState("");
  const [originalUrl, setOriginalUrl] = useState("");
  const [isSaving, setIsSaving] = useState(false);
  const fileInputRef = useRef<HTMLInputElement>(null);
  
  // Crop state
  const [isCropping, setIsCropping] = useState(false);
  const [cropImageUrl, setCropImageUrl] = useState("");
  const [crop, setCrop] = useState({ x: 0, y: 0 });
  const [zoom, setZoom] = useState(1);
  const [croppedAreaPixels, setCroppedAreaPixels] = useState<any>(null);
  const [croppedFile, setCroppedFile] = useState<File | null>(null);
  const [displayUrl, setDisplayUrl] = useState<string | null>(null);

  const isAvatar = field === 'avatar';

  useEffect(() => {
    if (id) {
      CommunityService.getCommunityById(id).then(c => {
        if (c) {
          setCommunity(c);
          const stored = c[field] || "";
          setImageUrl(stored);
          setOriginalUrl(stored);
          if (stored.startsWith('drive://') && isCommunityMediaRuntimeConfigured()) {
            const port = getCommunityMediaRuntime();
            port.resolveDisplayUrl?.(stored).then(url => {
              if (url) {
                setDisplayUrl(url);
              }
            }).catch(() => setDisplayUrl(null));
          }
        }
      });
    }
  }, [id, field]);

  const hasChanged = imageUrl !== originalUrl;

  const handleSave = async () => {
    if (!id || !community || !hasChanged) return;
    if (croppedFile && !isCommunityMediaRuntimeConfigured()) {
      showToast(t('community.auto_media_port_missing', '当前宿主未配置图片上传，无法保存'));
      return;
    }

    setIsSaving(true);
    try {
       let storedValue = imageUrl;
       if (croppedFile) {
         const urls = await getCommunityMediaRuntime().uploadImages([croppedFile]);
         storedValue = urls[0] ?? '';
         if (!storedValue) {
           throw new Error('image upload returned no URL');
         }
       }
       await CommunityService.updateCommunity(id, { [field]: storedValue });
       showToast(t('community.auto_fn_25b0deea', '保存成功'));
       navigate(-1);
    } catch {
       showToast(t('community.auto_fn_25b0066f', '保存失败'));
    } finally {
       setIsSaving(false);
    }
  };

  const handleUploadClick = () => {
  fileInputRef.current?.click();
  };

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
  const file = e.target.files?.[0];
    if (file) {
      if (fileInputRef.current) {
         fileInputRef.current.value = '';
      }
      const reader = new FileReader();
      reader.onload = (event) => {
         if (event.target?.result) {
            setCropImageUrl(event.target.result as string);
            setIsCropping(true);
            setCrop({ x: 0, y: 0 });
            setZoom(1);
         }
      };
      reader.readAsDataURL(file);
    }
  };

  const onCropComplete = useCallback((croppedArea: any, croppedAreaPixels_: any) => {
    setCroppedAreaPixels(croppedAreaPixels_);
  }, []);

  const handleConfirmCrop = async () => {
    try {
      const cropped = await getCroppedImg(cropImageUrl, croppedAreaPixels);
      setCroppedFile(cropped.file);
      setImageUrl(cropped.previewUrl);
      setIsCropping(false);
    } catch (e) {
      console.error(e)
    }
  };

  const titles = {
    avatar: t('community.auto_edit_avatar', '编辑头像'),
    coverImage: t('community.auto_edit_cover', '编辑背景')
  };

  if (isCropping) {
     return (
        <div className="flex flex-col h-full bg-black relative text-white">
           <header className="h-[56px] px-4 flex items-center justify-between shrink-0 pt-safe z-20 relative bg-black/50 backdrop-blur-md">
              <button 
                 onClick={() => setIsCropping(false)} 
                 className="text-[16px] text-white/80 active:opacity-70 p-2 -ml-2"
              >{t('community.auto_a9472', '取消')}</button>
              <h1 className="text-[17px] font-semibold flex-1 text-center">{t('community.auto_3f6a8e92', '裁剪图片')}</h1>
              <button 
                 onClick={handleConfirmCrop} 
                 className="text-[16px] text-blue-500 font-medium active:opacity-70 p-2 -mr-2"
              >{t('community.auto_ef0ec', '确定')}</button>
           </header>
           
           <div className="flex-1 relative w-full h-full bg-black">
             {/* @ts-ignore react-easy-crop typings */}
             <Cropper
                image={cropImageUrl}
                crop={crop}
                zoom={zoom}
                aspect={isAvatar ? 1 : 2}
                cropShape={isAvatar ? 'round' : 'rect'}
                showGrid={true}
                onCropChange={setCrop}
                onCropComplete={onCropComplete}
                onZoomChange={setZoom}
             />
           </div>
        </div>
     );
  }

  return (
    <div className="flex flex-col h-full bg-black relative text-white">
       <header className="h-[56px] px-4 flex items-center justify-between shrink-0 pt-safe z-20 relative bg-transparent">
          <div className="absolute left-4 z-10">
             <IconButton icon={<ChevronLeft className="w-6 h-6 text-white" />} className="bg-transparent w-10 h-10 -ml-2" onClick={() => navigate(-1)} />
          </div>
          <h1 className="text-[17px] font-semibold flex-1 text-center">{titles[field]}</h1>
          <div className="absolute right-4 z-10">
             <button 
                onClick={handleSave} 
                disabled={isSaving || !hasChanged} 
                className={cn(
                   "font-medium text-[15px] transition-opacity p-2 -mr-2",
                   hasChanged ? "text-blue-500 active:opacity-70" : "text-white/30"
                )}
             >{isSaving ? t('community.auto_saving', '保存中...') : t('community.auto_done', '完成')}</button>
          </div>
       </header>

       <div className="flex-1 flex flex-col items-center justify-center relative w-full overflow-hidden">
          <input 
             type="file" 
             ref={fileInputRef} 
             className="hidden" 
             accept="image/*" 
             onChange={handleFileChange} 
          />
          
          {/* Ambient blurred background */}
          {imageUrl ? (
             <div className="absolute inset-0 overflow-hidden z-0">
                <img src={displayUrl ?? imageUrl} alt="" className="w-full h-full object-cover opacity-20 blur-[60px] scale-125" />
             </div>
          ) : (
             <div className="absolute inset-0 bg-[#0a0a0a] z-0" />
          )}

          {/* Crop Area (Preview) */}
          <div className="relative z-10 w-full px-6 flex justify-center">
             <div className={cn(
                "relative flex items-center justify-center overflow-hidden border border-white/20 shadow-[0_0_40px_rgba(0,0,0,0.5)] bg-black/40 backdrop-blur-sm cursor-pointer active:scale-[0.98] transition-transform",
                isAvatar ? "w-[280px] h-[280px] rounded-full" : "w-full aspect-[2/1] rounded-2xl"
             )}
             onClick={handleUploadClick}>
                {imageUrl ? (
                   <img src={displayUrl ?? imageUrl} alt="Preview" className="w-full h-full object-cover" />
                ) : (
                   <div className="flex flex-col items-center gap-3">
                      <ImageIcon className="w-12 h-12 text-white/20" />
                      <span className="text-[14px] text-white/40">{t('community.auto_30225b27', '暂无图片')}</span>
                   </div>
                )}
             </div>
          </div>
       </div>

       <div className="pb-safe pt-8 pb-10 flex flex-col items-center shrink-0 w-full relative z-20 pointer-events-none">
          <p className="text-[14px] text-white/50">{t('community.auto_n7a576645', '点击图片更换{{target}}', { target: isAvatar ? t('community.auto_avatar', '头像') : t('community.auto_cover', '背景') })}</p>
       </div>
    </div>
  );
};
