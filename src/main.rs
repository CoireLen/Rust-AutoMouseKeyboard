mod mousekeyboard;
use fltk::{app, prelude::*, window::{Window},group::Flex,button::Button,input::Input,frame::Frame};
use std::boxed::Box;
use std::sync::{Arc,Mutex};
use opencv::imgcodecs::*;
use opencv::imgproc::{match_template,TM_SQDIFF,rectangle};
use opencv::highgui::*;
use opencv::core::*;
//1.屏幕截图 
//2.鼠标点击 mousebuttonup mousebuttonudown
//3.鼠标选择 mousemove
//4.键盘模拟 ctrl+c ctrl+v  
//5.键入模拟 set cilpboard ctrl+v
//6.图片定位 matchtemplate
//  - 6钟匹配方法
fn main() {
    let app = app::App::default();
    let winsize=(300,400);
    let mut wind = Window::new(100, 100, winsize.0, winsize.1, "注册工具");
    let mut flex0=Flex::default().with_size(winsize.0-10,winsize.1-10).center_of_parent().row();
    let mut flex1=Flex::default().with_size(winsize.0/2,winsize.1-10).center_of_parent().column();
    let mut labelname=Frame::default().with_label("姓名:");
    let mut labelpinyin=Frame::default().with_label("拼音/邮箱:");
    let mut labelid=Frame::default().with_label("工号:");
    let mut labelgroup=Frame::default().with_label("部门:");
    let mut labelgroupid=Frame::default().with_label("部门代号:");
    let mut buttongetregdata=Button::default().with_label("获取注册信息");
    let mut buttonregmail=Button::default().with_label("注册邮箱");
    flex1.end();
    let mut flex2=Flex::default().with_size(winsize.0/2,winsize.1-10).center_of_parent().column();
    let mut inputname=Arc::new(Mutex::new(Input::default()));
    let mut inputpinyin=Input::default();
    let mut inputid=Input::default();
    let mut inputgroup=Input::default();
    let mut inputgroupid=Input::default();
    let mut buttongetdoordata=Button::default().with_label("获取门禁信息");
    let mut buttonregtiptop=Button::default().with_label("注册Tiptop");
    flex2.end();
    flex0.end();
    wind.end();
    let inpt=Arc::clone(&inputname);
    buttongetregdata.set_callback(move|_|on_getregdata(&inpt));
    wind.show();
    app.run().unwrap();
}
fn on_getregdata(i:&Arc<Mutex<Input>>){
    println!("{}","获取注册信息");
    let sc=mousekeyboard::Screen::new();
    let mut mat=sc.getscreenshot();
    let (x,y)=sc.find_img(&mat,&"V:1.png");
    sc.mouse_move(x, y);
    //imshow("match", &mat);
    //wait_key(0);
}