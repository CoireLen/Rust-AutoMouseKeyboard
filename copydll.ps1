$ct=0
if((Test-Path .\target\debug\) -eq "True")
{
    $env:COPY_TARGET=".\target\debug\"
    $ct=1
}
if((Test-Path .\target\release\) -eq "True")
{
    $env:COPY_TARGET=".\target\release\"
    $ct=1
}
if ($ct -eq 1){
Copy-Item $env:OPENCV_LINK_PATHS/../bin/gif.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/jpeg62.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/leptonica-1.82.0.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/liblzma.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/libpng16.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/libprotobuf.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/libwebpmux.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/opencv_aruco.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/opencv_barcode.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/opencv_bgsegm.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/opencv_bioinspired.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/opencv_calib3d.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/opencv_ccalib.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/opencv_core.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/opencv_datasets.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/opencv_dnn.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/opencv_dnn_objdetect.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/opencv_dnn_superres.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/opencv_dpm.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/opencv_face.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/opencv_features2d.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/opencv_flann.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/opencv_fuzzy.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/opencv_hdf.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/opencv_hfs.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/opencv_highgui.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/opencv_imgcodecs.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/opencv_imgproc.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/opencv_img_hash.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/opencv_intensity_transform.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/opencv_line_descriptor.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/opencv_mcc.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/opencv_ml.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/opencv_objdetect.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/opencv_optflow.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/opencv_phase_unwrapping.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/opencv_photo.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/opencv_plot.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/opencv_quality.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/opencv_rapid.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/opencv_reg.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/opencv_saliency.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/opencv_shape.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/opencv_stereo.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/opencv_stitching.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/opencv_structured_light.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/opencv_superres.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/opencv_surface_matching.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/opencv_text.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/opencv_tracking.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/opencv_video.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/opencv_videoio.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/opencv_videostab.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/opencv_wechat_qrcode.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/opencv_xfeatures2d.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/opencv_ximgproc.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/opencv_xobjdetect.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/opencv_xphoto.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/openjp2.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/tesseract41.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/tiff.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/webp.dll $env:COPY_TARGET 
Copy-Item $env:OPENCV_LINK_PATHS/../bin/zlib1.dll $env:COPY_TARGET 
}