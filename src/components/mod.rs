/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

pub mod app;
pub mod editor;
pub mod getting_started;
pub mod sidebar;
pub mod window;

pub use app::EchidnaEditor;
pub use editor::EchidnaCoreEditor;
pub use getting_started::GettingStartedPage;
pub use sidebar::EchidnaSidebar;
pub use window::EchidnaWindow;

pub mod prelude {
    pub use super::window::{file::*, menubar::*};
}
