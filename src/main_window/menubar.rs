// -!- rust -!- //////////////////////////////////////////////////////////////
//
//  System        : 
//  Module        : 
//  Object Name   : $RCSfile$
//  Revision      : $Revision$
//  Date          : $Date$
//  Author        : $Author$
//  Created By    : Robert Heller
//  Created       : 2025-10-24 10:23:23
//  Last Modified : <251024.2048>
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
use iced::widget::{pick_list,row};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum File {
    New,
    Open,
    Save,
    SaveAs,
    Print,
    Close,
    Exit,
    None,
}

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::New => "New",
            Self::Open => "Open...",
            Self::Save => "Save...",
            Self::SaveAs => "Save As...",
            Self::Print => "Print...",
            Self::Close => "Close",
            Self::Exit  => "Exit",
            Self::None  => "File",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Edit {
    Undo,
    Cut,
    Copy,
    Clear,
    Delete,
    SelectAll,
    DeSelectAll,
    None
}

impl std::fmt::Display for Edit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Undo => "Undo",
            Self::Cut => "Cut",
            Self::Copy => "Copy",
            Self::Clear => "Clear",
            Self::Delete => "Delete",
            Self::SelectAll => "Select All",
            Self::DeSelectAll => "De-select All",
            Self::None => "Edit",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum View {
    ViewOneTrain,
    ViewAllTrains,
    ViewOneStation,
    ViewAllStations,
    ViewOneNote,
    ViewAllNotes,
    None,
}

impl std::fmt::Display for View {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::ViewOneTrain => "View One Train",
            Self::ViewAllTrains => "View All Trains",
            Self::ViewOneStation => "View One Station",
            Self::ViewAllStations => "View All Stations",
            Self::ViewOneNote => "View One Note",
            Self::ViewAllNotes => "View All Notes",
            Self::None => "View",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Options {
    EditSystemConfiguration,
    SaveSystemConfiguration,
    ReloadSystemConfiguration,
    None,
}

impl std::fmt::Display for Options {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::EditSystemConfiguration => "Edit System Configuration",
            Self::SaveSystemConfiguration => "Save System Configuration",
            Self::ReloadSystemConfiguration => "Re-load System Configuration",
            Self::None => "Options",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Trains {
    AddTrain,
    EditTrain,
    DeleteTrain,
    None,
}

impl std::fmt::Display for Trains {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::AddTrain => "Add Train",
            Self::EditTrain => "Edit Train",
            Self::DeleteTrain => "Delete Train",
            Self::None => "Trains",
        })
    }
} 

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stations {
    SetDuplicateStation,
    ClearDuplicateStation,
    AddStorageTrack,
    None,
}

impl std::fmt::Display for Stations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::SetDuplicateStation => "Set Duplicate Station",
            Self::ClearDuplicateStation => "Clear Duplicate Station",
            Self::AddStorageTrack => "Add Storage Track",
            Self::None => "Stations",
        })
    }
} 

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cabs {
    AddACab,
    None,
}

impl std::fmt::Display for Cabs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::AddACab => "Add A Cab",
            Self::None => "Cabs",
        })
    }
} 

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Notes {
    CreateNewNote,
    EditExistingNote,
    AddNoteToTrain,
    AddNoteToTrainAtStation,
    RemoveNoteFromTrain,
    RemoveNoteFromTrainAtStation,
    None,
}

impl std::fmt::Display for Notes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::CreateNewNote => "Create New Note",
            Self::EditExistingNote => "Edit Existing Note",
            Self::AddNoteToTrain => "Add note to train",
            Self::AddNoteToTrainAtStation => "Add Note To Train At Station Stop",
            Self::RemoveNoteFromTrain => "Remove Note From Train",
            Self::RemoveNoteFromTrainAtStation => "Remove Note From Train At Station Stop",
            Self::None => "Notes",
        })
    }
} 


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Help {
    OnHelp,
    OnKeys,
    Index,
    Tutorial,
    OnVersion,
    Warranty,
    Copying,
    None,
}

impl std::fmt::Display for Help {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::OnHelp => "On Help...",
            Self::OnKeys => "On Keys...",
            Self::Index => "Index...",
            Self::Tutorial => "Tutorial",
            Self::OnVersion => "On Version",
            Self::Warranty => "Warranty",
            Self::Copying => "Copying",
            Self::None => "Help",
        })
    }
}


