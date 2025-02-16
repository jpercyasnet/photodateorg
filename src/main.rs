use iced::alignment::{Alignment};
use iced::theme::{Theme};
use iced::widget::{
    button, checkbox, column, row, scrollable, text, horizontal_space,
    container, Column, Row, text_input, Radio, Space,
};
use iced::event::{self, Event};
use iced::Subscription;
use iced::window;
use iced::{Element};
use iced::{Center, Color, Task, Length, Size};

use serde::{Deserialize, Serialize};

mod get_winsize;
mod dump_file;
mod sourcedirpressm;
mod targetdirpressm;
mod updatepressm;
mod executechangepressm;

use get_winsize::get_winsize;
use sourcedirpressm::sourcedirpressm;
use targetdirpressm::targetdirpressm;
use updatepressm::updatepressm;
use executechangepressm::executechangepressm;
pub fn main() -> iced::Result {
     let mut widthxx: f32 = 1350.0;
     let mut heightxx: f32 = 750.0;
     let (errcode, _errstring, widtho, heighto) = get_winsize();
     if errcode == 0 {
         widthxx = widtho as f32 - 20.0;
         heightxx = heighto as f32 - 75.0;
     }
     iced::application(PhotoChg::title, PhotoChg::update, PhotoChg::view)
        .window_size((widthxx, heightxx))
        .theme(PhotoChg::theme)
        .subscription(PhotoChg::subscription)
        .run_with(PhotoChg::new)

}

#[derive(Debug)]
enum PhotoChg {
    Loaded(State),
    
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UseChoice {
    NON,
    DIN,
    SDD,
    TDD,
}

impl Default for UseChoice {
    fn default() -> Self {
        UseChoice::NON
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DtChoice {
    DT,
    OD,
    OT,
}

impl Default for DtChoice {
    fn default() -> Self {
        DtChoice::DT
    }
}

#[derive(Debug, Default)]
struct State {
    filterf: Filterf,
    files: Vec<File>,
    sourcedir_value: String,
    targetdir_value: String,
    msg_value: String,
    mess_color: Color,
    usechoice_value: UseChoice,
    dtchoice_value: DtChoice,
    screenwidth: f32,
    sourcemarked_bool: bool,
    dateinname_bool: bool,
    sourcedd_bool: bool,
    sourcedd_value: String,
    targetdd_bool: bool,
    targetdd_value: String,
    hhmmss1_value: String,
    keepoffset_bool: bool,
    updateall_bool: bool,
    execute_bool: bool,
}

#[derive(Debug, Clone)]
enum Message {
    FilterChangedf(Filterf),
    FileMessage(usize, FileMessage),
    SourceDirPressed,
    TargetDirPressed,
    UseRadioSelected(UseChoice),
    DtRadioSelected(DtChoice),
    Size(Size),
    SourceMarked(bool),
    Hhmmss1Changed(String),
    KeepOffset(bool),
    UpdateAllCheck(bool),
    ExecuteCheck(bool),
    UpdateAllPressed,
    UpdateSelectionPressed,
    ExecuteChangePressed,
}

impl PhotoChg {
    fn new() -> (Self, Task<Message>) {
        let mut widthxx: u32 = 1300;
        let (errcode, errstring, widtho, _heighto) = get_winsize();
        let for_message: String;
        if errcode == 0 {
            widthxx = widtho;
            for_message = format!("{}", errstring);
        } else {
            for_message = format!("**ERROR {} get_winsize: {}", errcode, errstring);
        }

        (
            PhotoChg::Loaded(State
               {
                filterf:Filterf::All,
                files:Vec::<File>::new(),
                sourcedir_value: "no directory".to_string(),
                targetdir_value: "no directory".to_string(),
                mess_color: Color::from([0.5, 0.5, 1.0]),
                msg_value: for_message.to_string(),
                usechoice_value:UseChoice::NON,
                dtchoice_value:DtChoice::DT,
                screenwidth: widthxx as f32,
                sourcemarked_bool: true,
                dateinname_bool: false,
                sourcedd_bool: false,
                sourcedd_value: "none".to_string(),
                targetdd_bool: false,
                targetdd_value: "none".to_string(),
                hhmmss1_value: "-00:00:00:00:00:00".to_string(),
                keepoffset_bool: false,
                updateall_bool: false,
                execute_bool: false,
                }
            ),
            Task::none(),
        )
    }

    fn title(&self) -> String {
        format!("Photo Date Organizer -- iced")
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match self {
            PhotoChg::Loaded(state) => {

                let command = match message {
                    Message::FilterChangedf(filterf) => {
                        state.filterf = filterf;

                        Task::none()
                    }
                    Message::FileMessage(i, file_message) => {
                        if let Some(file) = state.files.get_mut(i) {

                            file.update(file_message);

                               Task::none()
                        } else {
                            Task::none()
                        }
                    }

                    Message::Size(size) => {
                         state.screenwidth = size.width;
                         Task::none()
                    }
                    Message::SourceMarked(picked) => {state.sourcemarked_bool = picked; Task::none()
                    }
/*                     Message::DateInName(picked) => {state.dateinname_bool = picked; Task::none()
                    }
                   Message::SourceDirDate(picked) => {state.sourcedd_bool = picked; Task::none()
                    }
                    Message::TargetDirDate(picked) => {state.targetdd_bool = picked; Task::none()
                    }
                    Message::RenameWithDate(picked) => {state.renamewd_bool = picked; Task::none()
                    }
                    Message::LengthDesc(value) => {state.lengthdesc = value; Task::none()
                    }
*/
                    Message::Hhmmss1Changed(value) => {state.hhmmss1_value = value; Task::none()
                    }
                    Message::KeepOffset(picked) => {state.keepoffset_bool = picked; Task::none()
                    }
                    Message::UpdateAllCheck(picked) => {state.updateall_bool = picked; Task::none()
                    }
                    Message::ExecuteCheck(picked) => {state.execute_bool = picked; Task::none()
                    }
                    Message::ExecuteChangePressed => {
                        if !state.execute_bool {
                            state.msg_value = "Check Box was not set to execute change".to_string();
                            state.mess_color = Color::from([1.0, 0.0, 0.0]);
                        } else {
                            state.execute_bool = false;
                            let mut listfiles: Vec<String> = Vec::new();
                            for filesy in state.files.iter() {
                                 let  mut fileybool = 0;
                                 if filesy.completed {
                                     fileybool = 1;
                                 }
                                 let fileydesc = filesy.description.clone();
                                 listfiles.push(format!("{}|{}", fileybool, fileydesc));
                            }
                            let (errcode, errstr, listitems) = executechangepressm(listfiles, 
                                                                                   state.sourcedir_value.clone(),
                                                                                   state.targetdir_value.clone(),
                                                                                  ); 
                            state.msg_value = errstr.to_string();
                            if errcode == 0 {
                                state.files.clear();                         
                                let listitemlen = listitems.len();
                                let newtoi = listitemlen as i32 ;
                                for indexi in 0..newtoi {
                                     state
                                       .files
                                       .push(File::new(listitems[indexi as usize].clone()));
                                } 
                                state.mess_color = Color::from([0.0, 1.0, 0.0]);
                            } else {
                                state.mess_color = Color::from([1.0, 0.0, 0.0]);
                            }
                        }
                        Task::none()
                   }
                    Message::UpdateAllPressed => {
                        if !state.updateall_bool {
                            state.msg_value = "Check Box was not set to execute update all".to_string();
                            state.mess_color = Color::from([1.0, 0.0, 0.0]);
                        } else {
                            state.updateall_bool = false;
                            let mut listfiles: Vec<String> = Vec::new();
                            for filesy in state.files.iter() {
                                 let  mut fileybool = 0;
                                 if filesy.completed {
                                     fileybool = 1;
                                 }
                                 let fileydesc = filesy.description.clone();
                                 listfiles.push(format!("{}|{}", fileybool, fileydesc));
                            }
                            let all_bool: bool = true;
                            let (errcode, errstr, listitems) = updatepressm(all_bool, listfiles, 
                                                                                            state.sourcedir_value.clone(),
                                                                                            state.targetdir_value.clone(),
                                                                                            state.sourcedd_bool.clone(),
                                                                                            state.sourcedd_value.clone(),
                                                                                            state.targetdd_bool.clone(),
                                                                                            state.targetdd_value.clone(),
                                                                                            state.hhmmss1_value.clone(),
                                                                                            state.dateinname_bool.clone()
//                                                                                            state.renamewd_bool.clone(),
//                                                                                            state.lengthdesc.clone()
                                                                                           ); 
                            state.msg_value = errstr.to_string();
                            if errcode == 0 {
                                state.files.clear();                         
                                let listitemlen = listitems.len();
                                let newtoi = listitemlen as i32 ;
                                for indexi in 0..newtoi {
                                     state
                                       .files
                                       .push(File::new(listitems[indexi as usize].clone()));
                                } 
                                state.mess_color = Color::from([0.0, 1.0, 0.0]);
                            } else {
                                state.mess_color = Color::from([1.0, 0.0, 0.0]);
                            }
                        }
                        Task::none()
                    }
                    Message::UpdateSelectionPressed => {
                       let files_selected = state.files.iter().filter(|file| file.completed).count();
                       if files_selected < 1 {
                           state.msg_value = "no files selected".to_string();
                           state.mess_color = Color::from([1.0, 0.0, 0.0]);
                       } else {
                           let mut listfiles: Vec<String> = Vec::new();
                           for filesy in state.files.iter() {
                                let  mut fileybool = 0;
                                if filesy.completed {
                                       fileybool = 1;
                                }
                                let fileydesc = filesy.description.clone();
                                listfiles.push(format!("{}|{}", fileybool, fileydesc));
                           }
                           let all_bool: bool = false;
                           let (errcode, errstr, listitems) = updatepressm(all_bool, listfiles,
                                                                                            state.sourcedir_value.clone(),
                                                                                            state.targetdir_value.clone(),
                                                                                            state.sourcedd_bool.clone(),
                                                                                            state.sourcedd_value.clone(),
                                                                                            state.targetdd_bool.clone(),
                                                                                            state.targetdd_value.clone(),
                                                                                            state.hhmmss1_value.clone(),
                                                                                            state.dateinname_bool.clone()
//                                                                                            state.renamewd_bool.clone(),
//                                                                                            state.lengthdesc.clone()
                                                                                           ); 



                           state.msg_value = errstr.to_string();

                           if errcode == 0 {
                               state.files.clear();                         
                               let listitemlen = listitems.len();
                               let newtoi = listitemlen as i32 ;
                               for indexi in 0..newtoi {
                                   state
                                       .files
                                       .push(File::new(listitems[indexi as usize].clone()));
                               } 
                               state.mess_color = Color::from([0.0, 1.0, 0.0]);
//                               state.sourcemarked_bool = false;
//                               state.files.clear();                         
//                               state.sourcedir_value = newdir.to_string();
                           } else {
                               state.mess_color = Color::from([1.0, 0.0, 0.0]);
                           }
                        }
                        Task::none()
                    }

                    Message::SourceDirPressed => {
                        if !state.sourcemarked_bool {
                            state.msg_value = "Check Box was not set to get Source Directory".to_string();
                            state.mess_color = Color::from([1.0, 0.0, 0.0]);
                        } else {
                            let (errcode, errstr, newdir, listitems, sourcedate) = sourcedirpressm(state.sourcedir_value.clone(), state.dateinname_bool.clone());
                            state.msg_value = errstr.to_string();
                            if errcode == 0 {
                                state.sourcemarked_bool = false;
                                state.files.clear();                         
                                state.sourcedir_value = newdir.to_string();
                                state.sourcedd_value = sourcedate.to_string();
                                let listitemlen = listitems.len();
                                let newtoi = listitemlen as i32 ;
                                for indexi in 0..newtoi {
                                    state
                                        .files
                                        .push(File::new(listitems[indexi as usize].clone()));
                                } 
                                state.mess_color = Color::from([0.0, 1.0, 0.0]);
                            } else {
                                state.mess_color = Color::from([1.0, 0.0, 0.0]);
                            }
                        }
                        Task::none()
                    } 
                    Message::TargetDirPressed => {
                        let (errcode, errstr, newdir, targetdate) = targetdirpressm(state.sourcedir_value.clone());
                        if errcode == 0 {
                            state.targetdir_value = newdir.to_string();
                            state.mess_color = Color::from([0.0, 1.0, 0.0]);
                            state.msg_value = "got target Directory".to_string();
                            state.targetdd_value = targetdate.to_string();
                        } else {
                            state.mess_color = Color::from([1.0, 0.0, 0.0]);
                            state.msg_value = errstr.to_string();
                        }
                        Task::none()
                    } 
                    Message::UseRadioSelected(xchoice) => {
                        let strx: String;
                        state.usechoice_value = xchoice;
                        state.mess_color = Color::from([0.0, 1.0, 0.0]);
                        state.dateinname_bool = false;
                        state.sourcedd_bool = false;
                        state.targetdd_bool = false;
                        match xchoice {
                           UseChoice::NON => strx = "choice none selected".to_string(),
                           UseChoice::DIN => {
                               strx ="choice date in name selected".to_string();
                               state.dateinname_bool = true;
                           },
                           UseChoice::SDD => {
                               if state.sourcedd_value == "none".to_string() {
                                   strx = format!("choice source dir date selected but invalid value of source dir of: {}", state.sourcedd_value);
                                   state.mess_color = Color::from([1.0, 0.0, 0.0]);
                                   state.usechoice_value = UseChoice::NON;
                               } else {
                                   strx ="choice date in name selected".to_string();
                                   state.sourcedd_bool = true;
                               }
                           },
                           UseChoice::TDD => {
                               if state.targetdd_value == "none".to_string() {
                                   strx = format!("choice target dir date selected but invalid value of source dir of: {}", state.targetdd_value);
                                   state.mess_color = Color::from([1.0, 0.0, 0.0]);
                                   state.usechoice_value = UseChoice::NON;
                               } else {
                                   strx ="choice date in name selected".to_string();
                                   state.targetdd_bool = true;
                               }
                           }
                        }
                        state.msg_value = strx;
                        Task::none()
                    }
                    Message::DtRadioSelected(dchoice) => {
                        let strx = match dchoice {
                        DtChoice::DT => "choice date and time selected",
                        DtChoice::OD => "choice only date selected",
                        DtChoice::OT => "choice only time selected" };
                       state.dtchoice_value = dchoice;
                       state.msg_value = strx.to_string();
                       Task::none()
                    }

                };

                Task::batch(vec![command, Task::none()])
            }
        }
    }

    fn view(&self) -> Element<Message> {
        match self {
            PhotoChg::Loaded(State {
                filterf,
                files,
                sourcedir_value,
                targetdir_value,
                msg_value,
                mess_color,
                usechoice_value,
                dtchoice_value,
                screenwidth,
                sourcemarked_bool,
//                dateinname_bool,
//                sourcedd_bool,
                sourcedd_value,
//                targetdd_bool,
                targetdd_value,
//                renamewd_bool,
//                lengthdesc,
                hhmmss1_value,
                keepoffset_bool,
                updateall_bool,
                execute_bool,
              ..
            }) => {
                let mut messcol = Column::new().spacing(10);
                messcol = messcol.push(container(row![text("Message:").size(20),
                 text(msg_value).size(20).color(*mess_color),
            ].align_y(Alignment::Center).spacing(10).padding(5)
                    ));

                let mut dirbutshow = Column::new().spacing(10);
                let dirspace = 5.0;
                dirbutshow = dirbutshow.push(container(row![container(row![checkbox(" ", *sourcemarked_bool)
                                                                       .on_toggle(Message::SourceMarked,),
                                                            button("Source Directory Button")
                                                             .on_press(Message::SourceDirPressed),
                                                            text(sourcedir_value)
                                                             .size(20)].spacing(10)).width(Length::Fill),
                                                             Space::with_width(Length::Fixed(dirspace)),
                                                             container(row![button("Target Directory Button")
                                                             .on_press(Message::TargetDirPressed),
                                                            text(targetdir_value)
                                                             .size(20)].spacing(10)).width(Length::Fill),
                                                           ].align_y(Alignment::Center).spacing(10).padding(1),
                 ));

 
                let controlsf = view_controlsf(files, *filterf);
                let filtered_files =
                    files.iter().filter(|file| filterf.matches(file));

                let mut filescol1 = Column::new().spacing(10);
                let mut n = 0;
                if filtered_files.clone().count() == 0 {
                    filescol1 = filescol1.push(container(row![empty_message(match filterf {
                        Filterf::All => "No directory selected or no files in directory",
                        Filterf::Active => "All files have been selected",
                        Filterf::Completed => "No files have been selected" 
                    })]));
                } else {
                    for filesy in files.iter() {
                         if filesy.completed {
                             if (filterf == &Filterf::All) || (filterf == &Filterf::Completed) {
                                 filescol1 = filescol1.push(container(row![filesy.view(n).map(move |message| {
                                    Message::FileMessage(n, message)
                                   })]));
                             }
                         } else {
                             if (filterf == &Filterf::All) || (filterf == &Filterf::Active) {
                                 filescol1 = filescol1.push(container(row![filesy.view(n).map(move |message| {
                                    Message::FileMessage(n, message)
                                   })]));
                             }
                         }
                         n = n + 1;
                    }
                }
                let mut filesrow = Row::new().spacing(20);
                filesrow = filesrow.push(container(filescol1).padding(10).width(Length::Fixed(1000.0)));

                let scrollable_contentf: Element<Message> =
                  Element::from(scrollable(
                    filesrow
                )
                .height(Length::Fill)
                .width(Length::Fill)
                .direction({
                    let scrollbar = scrollable::Scrollbar::new()
                        .width(10)
                        .margin(10)
                        .scroller_width(10);

                    scrollable::Direction::Both {
                        horizontal: scrollbar,
                        vertical: scrollbar,
                    }
                 })
                ); 

                let mut contentdatei= Column::new().spacing(10).padding(10);
                contentdatei = contentdatei.push(container(row![container(row![
                                                         text("Source Dir Date: "),
                                                         text(sourcedd_value),].spacing(10)).width(Length::Fill),
                                                        Space::with_width(Length::Fixed(dirspace)),
                                                        container(row![
                                                         text("Target Dir Date: "),
                                                         text(targetdd_value)].spacing(10)).width(Length::Fill),
                                                        ].align_y(Alignment::Center).spacing(10).padding(1),
                 ));

                let contentab = row![
                                     text("Offset (-YY:MM:DD:hh:mm:ss): "),
                                     text_input("-00:00:00:00:00:00", hhmmss1_value)
                                          .on_input(Message::Hhmmss1Changed).padding(10).width(200),
                                     checkbox("Keep Offset", *keepoffset_bool).on_toggle(Message::KeepOffset,),
                                    ].spacing(20).padding(1);

                let selected_usechoice = Some(usechoice_value);
                let ua = Radio::new(
                         "None",
                         UseChoice::NON,
                         selected_usechoice.copied(),
                         Message::UseRadioSelected,
                ).size(15);
                let ub = Radio::new(
                         "Use Date in Name",
                         UseChoice::DIN,
                         selected_usechoice.copied(),
                         Message::UseRadioSelected,
                ).size(15);
           
                let uc = Radio::new(
                         "Use Source Dir Date",
                         UseChoice::SDD,
                         selected_usechoice.copied(),
                         Message::UseRadioSelected,
                ).size(15);
           
                let ud = Radio::new(
                           "Use Target Dir Date",
                           UseChoice::TDD,
                           selected_usechoice.copied(),
                           Message::UseRadioSelected
                ).size(15);
           
                let contentuse = row![ua, ub, uc, ud, horizontal_space(),].spacing(80).padding(1);

                let selected_dtchoice = Some(dtchoice_value);
                let da = Radio::new(
                         "Date & Time",
                         DtChoice::DT,
                         selected_dtchoice.copied(),
                         Message::DtRadioSelected,
                ).size(15);
                let db = Radio::new(
                         "Only Date",
                         DtChoice::OD,
                         selected_dtchoice.copied(),
                         Message::DtRadioSelected,
                ).size(15);
           
                let dc = Radio::new(
                         "Only Time",
                         DtChoice::OT,
                         selected_dtchoice.copied(),
                         Message::DtRadioSelected,
                ).size(15);
           
                let contentdt = row![da, db, dc,].spacing(80).padding(1);

                let winwidth: f32 = screenwidth - 20.0;

                let columntob = column![controlsf, scrollable_contentf].width(Length::Fill);


                column![messcol, dirbutshow, contentdatei, contentuse, contentdt, contentab, 
                        row![horizontal_space(), button("Update Selection").on_press(Message::UpdateSelectionPressed)],
                        columntob,
                        row![checkbox(" ", *execute_bool).on_toggle(Message::ExecuteCheck,),
                             button("Execute Change").on_press(Message::ExecuteChangePressed),horizontal_space(),
                             checkbox(" ", *updateall_bool).on_toggle(Message::UpdateAllCheck,),
                             button("Update All").on_press(Message::UpdateAllPressed),]
                             ]
                         .spacing(1)
                         .max_width(winwidth)
                         .padding(10)
                         .into()
               
            }
        }
    }
    fn theme(&self) -> Theme {
         Theme::Dracula
    }

    fn subscription(&self) -> Subscription<Message> {
        event::listen_with(|event, _status, _window| match event {
            Event::Window(window::Event::Resized(size)) => {
                Some(Message::Size(size))
            }
            _ => None,
        })
    }

}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct File {
    description: String,
    completed: bool,

}

#[derive(Debug, Clone)]
pub enum FileMessage {
    Completed(bool),
}

impl File {

    fn new(description: String) -> Self {
        File {
            description,
            completed: false,
        }
    }

    fn update(&mut self, message: FileMessage) {
        match message {
            FileMessage::Completed(completed) => {
                self.completed = completed;
            }

        }
    }

    fn view(&self, _i: usize) -> Element<FileMessage> {
                let checkbox = checkbox(
                    &self.description,
                    self.completed).on_toggle(FileMessage::Completed).width(Length::Fixed(1000.0));

                row![
                    checkbox,

                ]
                .spacing(20)
                .align_y(Alignment::Center)
                .into()

    }
}


fn view_controlsf(files: &[File], current_filter: Filterf) -> Element<Message> {
    let files_left = files.iter().filter(|file| file.completed).count();

    let filter_button = |label, filterf, current_filter| {
        let label = text(label).size(16);

        let button = button(label).style(if filterf == current_filter {
            button::primary
        } else {
            button::text
        });

        button.on_press(Message::FilterChangedf(filterf)).padding(8)
    };

        row![Space::with_width(Length::Fixed(20.0)),
            text(format!(
            "{} {} selected",
            files_left,
            if files_left == 1 { "file" } else { "files" }
        ))
        .size(16),

            filter_button("All", Filterf::All, current_filter),
            filter_button("Not Selected", Filterf::Active, current_filter),
            filter_button("Selected", Filterf::Completed, current_filter,),
        ]
        .width(Length::Shrink)
        .spacing(10)
    .align_y(Alignment::Center)
    .into()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl Default for Filter {
    fn default() -> Self {
        Filter::All
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Filterf {
    All,
    Active,
    Completed,
}

impl Default for Filterf {
    fn default() -> Self {
        Filterf::All
    }
}

impl Filterf {
    fn matches(&self, file: &File) -> bool {
        match self {
            Filterf::All => true,
            Filterf::Active => !file.completed,
            Filterf::Completed => file.completed,
        }
    }
}

fn empty_message(message: &str) -> Element<'_, Message> {
    container(
        text(message)
            .width(Length::Fill)
            .size(25)
            .align_x(Center)
            .color([0.7, 0.7, 0.7]),
    )
    .width(Length::Fill)
    .height(Length::Fixed(200.0))
    .into()
}
