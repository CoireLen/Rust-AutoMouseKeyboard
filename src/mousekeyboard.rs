use windows::{
    core::*, 
    Win32::UI::WindowsAndMessaging::*,
    Win32::UI::Input::KeyboardAndMouse::*,
    Win32::Graphics::Gdi::*,
    Win32::Foundation::*
};
use std::{time,thread,mem};
use opencv::prelude::*;
use opencv::imgcodecs::*;
use opencv::imgproc::{match_template,TM_SQDIFF,rectangle};
use opencv::core::*;
pub struct Screen{
    pub x:i32,
    pub y:i32,
    pub screenx:f64,
    pub screeny:f64,
    pub zoom:f64
}
impl Screen{
    pub fn new()->Screen{
        let mut sc=Screen{x:0,y:0,screenx:0f64,screeny:0f64,zoom:0f64};
        unsafe{
            sc.x=GetSystemMetrics(SM_CXSCREEN);
            sc.y=GetSystemMetrics(SM_CYSCREEN);
        }
        sc.screenx=65535f64/sc.x as f64;
        sc.screeny=65535f64/sc.y as f64;
        //获取屏幕长宽
        sc.zoom=1.0;
        sc
    }
    pub fn mouse_move(&self,x:i32,y:i32){
        let mf=MOUSEEVENTF_ABSOLUTE|MOUSEEVENTF_MOVE;
        unsafe{
            mouse_event(mf, (x as f64*self.screenx)as i32, (y as f64*self.screeny) as i32, 0, 0);
        }
        println!("Mouse Move {},{}",x,y);
    }
    pub fn mouse_click(&self,x:i32,y:i32){
        self.mouse_move(x, y);
        let mf=MOUSEEVENTF_LEFTDOWN|MOUSEEVENTF_LEFTUP;
        unsafe{
            mouse_event(mf, 0, 0, 0, 0);
        }
        println!("Mouse Click");
    }
    pub fn mouse_select(&self,x1:i32,y1:i32,x2:i32,y2:i32){
        self.mouse_move(x1, y1);
        let mut mf=MOUSEEVENTF_LEFTDOWN;
        unsafe{
            mouse_event(mf, 0, 0, 0, 0);
        }
        self.mouse_move(x2, y2);
        mf=MOUSEEVENTF_LEFTUP;
        unsafe{
            mouse_event(mf, 0, 0, 0, 0);
        }
        println!("Mouse Select");
    }
    pub fn getscreenshot(&self)->Mat{
        let mat;
        let mut data: Vec<u8> = vec![];
        println!("获取桌面截图");
        unsafe{
            let hwnd = GetDesktopWindow();
            let dc = GetWindowDC(hwnd);
            let cdc = CreateCompatibleDC(HDC::default());
            let mut rect = RECT {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            };
            GetWindowRect(hwnd, &mut rect);
            let (w, h) = (rect.right, rect.bottom);
            let bm = CreateCompatibleBitmap(dc, w, h);
            SelectObject(cdc, bm );
            StretchBlt(cdc, 0, 0, w, h, dc, 0, 0, w, h, SRCCOPY);
            let buf = vec![0u8; (w * h * 4) as usize];
            GetObjectW(bm , 84, buf.as_ptr() as *mut core::ffi::c_void);
            let mut bi = BITMAPINFO::default();
            bi.bmiHeader.biBitCount=32;
            bi.bmiHeader.biWidth=w;
            bi.bmiHeader.biHeight=h;
            bi.bmiHeader.biPlanes=1;
            bi.bmiHeader.biSize=mem::size_of_val(&bi.bmiHeader) as u32;
            CreateDIBSection(cdc,&bi,DIB_RGB_COLORS,buf.as_ptr() as *mut *mut core::ffi::c_void,HANDLE::default(),0);
            GetDIBits(cdc,bm, 0,h as u32,buf.as_ptr() as *mut core::ffi::c_void,&mut bi,DIB_RGB_COLORS,);
            let bif = BITMAPFILEHEADER {
                bfType: ('B' as u16) | (('M' as u16) << 8),
                bfOffBits: 54,
                bfReserved1: 0,
                bfReserved2: 0,
                bfSize: (w * h * 4 + 54) as u32,
            };
            for v in serialize_row(&bif) {
                data.push(*v);
            }
            let bii = BITMAPINFOHEADER {
                biBitCount: 32,
                biSize: 40,
                biWidth: w,
                biHeight: h,
                biPlanes: 1,
                biCompression: 0,
                biSizeImage: (w * h * 4) as u32,
                biClrImportant: 0,
                biClrUsed: 0,
                biXPelsPerMeter: 0,
                biYPelsPerMeter: 0,
            };
            for v in serialize_row(&bii) {
                data.push(*v);
            }
            for v in buf {
                data.push(v);
            }
            let vc=Vector::from_slice(data.as_slice());
            mat=imdecode(&vc, IMREAD_ANYCOLOR);
        }
        mat.unwrap()
    }
    pub fn find_img(&self,img:&Mat,filename:&str)->(i32,i32){
        let mat1=imread(filename, IMREAD_ANYCOLOR).unwrap();
        let mut res;
        unsafe{
            res=Mat::new_size(Size::new(100,100), CV_32FC1).unwrap();
        }
        match_template(img,&mat1,&mut res,TM_SQDIFF,&no_array());
        let (mut min_val,mut max_val)=(0f64,0f64);
        let (mut min_loc,mut max_loc)=(Point::new(0,0),Point::new(0,0));
        min_max_loc(&res, Some(&mut min_val), Some(&mut max_val),Some(&mut min_loc), Some(&mut max_loc), &no_array());
        let (mut isx,mut isy)=(0,0);
        if let Ok(imgsize)=mat1.size(){
            isx=imgsize.width;
            isy=imgsize.height;
        }
        (min_loc.x+isx/2,min_loc.y+isy/2)
    }
}
pub fn ctrl_(key:u8){
    unsafe{
        keybd_event(VK_CONTROL.0 as u8, 0, KEYBD_EVENT_FLAGS::default(), 0);
        keybd_event(key,0,KEYBD_EVENT_FLAGS::default(),0);
        thread::sleep_ms(10);
        keybd_event(VK_CONTROL.0 as u8, 0, KEYEVENTF_KEYUP, 0);
        keybd_event(key,0,KEYEVENTF_KEYUP,0);
        println!("Ctrl_{}",key as char)
    }
}
pub unsafe fn serialize_row<T: Sized>(src: &T) -> &[u8] {
    ::std::slice::from_raw_parts((src as *const T) as *const u8, ::std::mem::size_of::<T>())
}