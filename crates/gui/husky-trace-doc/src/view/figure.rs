use super::*;
use egui::{load::Bytes, pos2, Frame, Image, ImageSource, Rect};
use husky_trace_protocol::figure::IsFigure;

impl<'a, TraceProtocol, Settings> TraceDocView<'a, TraceProtocol, Settings>
where
    TraceProtocol: IsTraceProtocol,
    <TraceProtocol::Figure as IsFigure>::View<'a>: egui::Widget,
    Settings: HasTraceViewDocSettings,
{
    pub(super) fn render_figure(&mut self, ui: &mut egui::Ui) {
        if let Some(figure) = self.figure {
            figure
                .view(self.trace_synchrotron.visual_synchrotron())
                .ui(ui);
        }
    }
}
