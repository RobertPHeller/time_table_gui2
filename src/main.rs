// -!- rust -!- //////////////////////////////////////////////////////////////
//
//  System        : 
//  Module        : 
//  Object Name   : $RCSfile$
//  Revision      : $Revision$
//  Date          : $Date$
//  Author        : $Author$
//  Created By    : Robert Heller
//  Created       : 2025-10-27 11:36:17
//  Last Modified : <251028.1409>
//
//  Description	
//
//  Notes
//
//  History
//	
/////////////////////////////////////////////////////////////////////////////
//    Copyright (C) 2025  Robert Heller D/B/A Deepwoods Software
//			51 Locke Hill Road
//			Wendell, MA 01379-9728
//
//    This program is free software; you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation; either version 2 of the License, or
//    (at your option) any later version.
//
//    This program is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    You should have received a copy of the GNU General Public License
//    along with this program; if not, write to the Free Software
//    Foundation, Inc., 675 Mass Ave, Cambridge, MA 02139, USA.
// 
//
//////////////////////////////////////////////////////////////////////////////

#![allow(unused_imports)]
#![doc = include_str!("../README.md")]

use std::collections::BTreeMap; 
use iced::{time, window, Center, Element, Fill, Subscription, Task, Theme};    
use iced::widget::horizontal_space;

mod main_window;
mod file_open_select;

struct Application {
    windows: BTreeMap<window::Id, Window>,
    grab: Option<window::Id>,
}

#[derive(Debug, Clone)]
pub enum WindowType {
    MainWindow,
    FileSelectDialog(String,String,String,String,main_window::Message,window::Id),
}



impl WindowType {
    /// The [`window::Settings`] for each type of window.
    pub fn window_settings(&self) -> window::Settings {
        match self {
            WindowType::MainWindow => window::Settings::default(),
            WindowType::FileSelectDialog(defext,initdir,initfile,title,_,_) => file_open_select::Manager::Settings(),
        }
    }

    /// Create the appropriate [`Window`] for this type.
    fn create(&self) -> Window {
        match self {
            WindowType::MainWindow => Window::MainWindow(main_window::Manager::default()),
            WindowType::FileSelectDialog(defext,initdir,initfile,title,return_message,return_id) => Window::FileOpenDialog(file_open_select::Manager::Setup(defext,initdir,initfile,title),return_message,return_id)
        }
    }
}

#[derive(Clone)] 
pub enum Window {
    MainWindow(main_window::Manager),
    FileOpenDialog(file_open_select::Manager,impl Fn(String) -> main_window::Message,window::Id),
}

#[derive(Debug, Clone)] 
pub enum Message {
    // Messages handled at the top level
    WindowOpened(window::Id, WindowType),
    WindowClosed(window::Id),

    // Messages for the different types of windows
    MainWindow(window::Id, main_window::Message),
    FileOpenDialog(window::Id, file_open_select::Message),
}

impl Application {
    fn new() -> (Self, Task<Message>) {
        let window_type = WindowType::MainWindow;
        let (_, open) = window::open(window_type.window_settings()); 
        (
            Self {
                windows: BTreeMap::new(),
                grab: None,
            },
            open.map(move |id| Message::WindowOpened(id, window_type.clone())),
        )
    }

    fn title(&self, id: window::Id) -> String {
        match self.windows.get(&id) {
            Some(Window::MainWindow(_)) => "Time Table V2".to_string(),
            Some(Window::FileOpenDialog(_)) => "File Select".to_string(),
            None => "Unknown Window".to_string(),
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::WindowOpened(id, window_type) => {
                let window = window_type.create();
                match window_type {
                    WindowType::FileSelectDialog(..) => {
                        self.grab = Some(id);
                    },
                    _ => (),
                }
                self.windows.insert(id, window);
            }
            Message::WindowClosed(id) => {
                if self.grab == Some(id) {
                    self.grab = None;
                }
                self.windows.remove(&id);

                if self.windows.is_empty() {
                    return iced::exit();
                }
            }
            Message::MainWindow(id, message) if self.grab.is_none() || 
                                                self.grab == Some(id) => {
                if let Some(Window::MainWindow(manager)) = self.windows.get_mut(&id) {
                    if let Some(action) = manager.update(message) {
                        match action {
                            main_window::Action::OpenFileSelect(defext,initdir,initfile,title,return_message)
                                    => {
                                let window_type = WindowType::FileSelectDialog(defext,initdir,initfile,title,return_message,id);
                                let (_, open) = window::open(window_type.window_settings());
                                return open
                                    .map(move |id| Message::WindowOpened(id, window_type.clone()));
                            },
                        }
                    }
                }
            }
            Message::FileOpenDialog(id, message) if self.grab.is_none() || 
                                                self.grab == Some(id) => {
                if let Some(Window::FileOpenDialog(manager,retmes,retid)) = self.windows.get_mut(&id) {
                    if let Some(action) = manager.update(message) {
                        self.grab = None;
                        match action {
                            file_open_select::Action::Cancel => {
                                let _ = window::close(id);
                            },
                            file_open_select::Action::Open(filename) => {
                                if let Some(Window::MainWindow(mmanager)) = self.windows.get_mut(&retid) {
                                    let _ = mmanager.update(retmes(filename));
                                }
                                let _ = window::close(id);
                            },
                        }
                    }
                }
            }
        }

        Task::none()
    }

    fn view(&self, id: window::Id) -> Element<'_, Message> {
        if let Some(window) = self.windows.get(&id) {
            match &window {
                Window::MainWindow(manager) => manager
                    .view()
                    .map(move |message| Message::MainWindow(id, message)),
                Window::FileOpenDialog(manager, ..) => manager
                    .view() 
                    .map(move |message| Message::FileOpenDialog(id, message)),
            }
        } else {
            horizontal_space().into()
        }
    }

    fn theme(&self, _: window::Id) -> Theme {
        Theme::Light
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            window::close_events().map(Message::WindowClosed),
        ])
    }
}

pub fn main() -> iced::Result {
    iced::daemon(Application::title, Application::update, Application::view)
        .subscription(Application::subscription)
        .theme(Application::theme)
        .run_with(Application::new)
}

        
    
