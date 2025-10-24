// -!- rust -!- //////////////////////////////////////////////////////////////
//
//  System        : 
//  Module        : 
//  Object Name   : $RCSfile$
//  Revision      : $Revision$
//  Date          : $Date$
//  Author        : $Author$
//  Created By    : Robert Heller
//  Created       : 2025-10-24 10:24:21
//  Last Modified : <251024.1340>
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


mod menubar;
use crate::menubar::*;
use iced::widget::{pane_grid, pick_list, button, column, row, text, Column,
                   scrollable, responsive, container}; 
use iced::widget::pane_grid::{Axis,PaneGrid};
use iced::{Theme,Center, Color, Element, Fill, Size, Subscription};
use iced::keyboard;
use std::process::exit;

#[derive(Debug, Clone, Copy)]
enum Message {
    Split(pane_grid::Axis, pane_grid::Pane),
    SplitFocused(pane_grid::Axis),
    FocusAdjacent(pane_grid::Direction),
    Clicked(pane_grid::Pane),
    Dragged(pane_grid::DragEvent),
    Resized(pane_grid::ResizeEvent),
    TogglePin(pane_grid::Pane),
    Maximize(pane_grid::Pane),
    Restore,
    Close(pane_grid::Pane),
    CloseFocused,
    FileSelected(File),
    EditSelected(Edit),
    ViewSelected(View),
    OptionsSelected(Options),
    HelpSelected(Help),
}

struct TimeTableGUI {
    panes: pane_grid::State<Pane>,
}

impl TimeTableGUI {
    fn new() -> Self {
        let (mut panes, main) = pane_grid::State::new(Pane::Main);
        let other = panes.split(Axis::Vertical,main,Pane::Buttons);
        TimeTableGUI {
            panes, 
        }
    }
    fn update(&mut self, message: Message) {
        match message {
            Message::FileSelected(File::Exit) => exit(0),
            _ => (),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let pane_grid = pane_grid(&self.panes, |pane, state, is_maximized| {
            pane_grid::Content::new(match state {
                Pane::Main => text("This is the Main pane"),
                Pane::Buttons => text("This is the Buttons pane"),
            })
        });

        let filemenuitems = [
            File::New,
            File::Open,
            File::Save,
            File::SaveAs,
            File::Close,
            File::Exit,
        ];        
        let editmenuitems = [
            Edit::Undo,
            Edit::Cut,
            Edit::Copy,
            Edit::Clear,
            Edit::Delete,
            Edit::SelectAll,
            Edit::DeSelectAll,
        ];
        let viewmenuitems = [
        ];
        let optionsmenuitems = [
        ];
        let helpmenuitems = [
            Help::OnHelp,
            Help::OnKeys,
            Help::Index,
            Help::Tutorial,
            Help::OnVersion,
            Help::Warranty,
            Help::Copying,
        ];
        column![
            row![pick_list(filemenuitems,
                          Some(File::None),
                          Message::FileSelected,
                 ), 
                 pick_list(editmenuitems,
                            Some(Edit::None),
                            Message::EditSelected,
                 ), 
                 pick_list(viewmenuitems,
                            Some(View::None),
                            Message::ViewSelected,
                 ), 
                 pick_list(optionsmenuitems,
                            Some(Options::None),
                            Message::OptionsSelected,
                 ), 
                 pick_list(helpmenuitems,
                            Some(Help::None),
                            Message::HelpSelected,
                 ),
            ],
             container(pane_grid)
                .width(Fill)
                .height(Fill)
                .padding(0),
        ].into()
    }
}

impl Default for TimeTableGUI {
    fn default() -> Self {
        TimeTableGUI::new()
    }
}

#[derive(Clone, Copy)]
enum Pane {
    Main,
    Buttons,
}


pub fn main() -> iced::Result {
    iced::application("Time Table 2", TimeTableGUI::update, TimeTableGUI::view)
        .theme(|_| Theme::Light)
        .centered()
        .run()
}
