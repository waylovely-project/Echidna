/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */
use crate::prelude::*;
pub use adw::subclass::prelude::*;

use gtk::glib::clone;
use gtk::CompositeTemplate;
use std::cell::RefCell;

#[derive(Debug, Default, CompositeTemplate)]
#[template(resource = "/io/fortressia/Echidna/window.ui")]
pub struct EchidnaWindow {
    #[template_child]
    pub tab_bar: TemplateChild<adw::TabBar>,
    #[template_child]
    pub sidebar: TemplateChild<super::super::sidebar::EchidnaSidebar>,
    pub dialogs: RefCell<Vec<gtk::NativeDialog>>,
    #[template_child]
    pub split_button: TemplateChild<adw::SplitButton>,
}

#[glib::object_subclass]
impl ObjectSubclass for EchidnaWindow {
    const NAME: &'static str = "EchidnaWindow";
    type Type = super::EchidnaWindow;
    type ParentType = adw::ApplicationWindow;

    fn class_init(class: &mut Self::Class) {
        Self::bind_template(class);
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for EchidnaWindow {
    fn constructed(&self, win: &Self::Type) {
        self.split_button.connect_clicked(clone!(@weak win =>
            move |_button| {
                win.action_open_file();
        }));
    }
}

impl WidgetImpl for EchidnaWindow {}

impl WindowImpl for EchidnaWindow {}

impl ApplicationWindowImpl for EchidnaWindow {}

impl AdwApplicationWindowImpl for EchidnaWindow {}

impl BuildableImpl for EchidnaWindow {}
