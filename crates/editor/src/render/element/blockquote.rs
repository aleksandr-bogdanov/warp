use warpui::{AppContext, geometry::vector::vec2f};

use crate::{
    extract_block,
    render::model::{BlockItem, RenderState, viewport::ViewportItem},
};

use super::{RenderContext, RenderableBlock, placeholder::BlockPlaceholder};

/// Width of the left accent bar painted for blockquote blocks.
const BLOCKQUOTE_BAR_WIDTH: f32 = 3.;

/// Renders a blockquote block: a left accent bar + optional background fill,
/// with text rendered via the inner [`ParagraphBlock`] (delegating typography
/// to the standard rich-text path so blockquotes inherit body font/spacing).
///
/// Mirrors the pattern of [`RenderableRunnableCommand`] (bg + border + content)
/// and [`RenderableBulletList`] (decorative shape + text), keeping each block
/// type's visual treatment in its own dedicated render element. Uses
/// `ParagraphBlock` so multi-line `> foo\n> bar` blockquotes render under one
/// continuous bar instead of stacking separate bars per line.
pub struct RenderableBlockquote {
    viewport_item: ViewportItem,
    placeholder: BlockPlaceholder,
}

impl RenderableBlockquote {
    pub fn new(viewport_item: ViewportItem) -> Self {
        Self {
            viewport_item,
            placeholder: BlockPlaceholder::new(true),
        }
    }
}

impl RenderableBlock for RenderableBlockquote {
    fn viewport_item(&self) -> &ViewportItem {
        &self.viewport_item
    }

    fn layout(&mut self, _model: &RenderState, _ctx: &mut warpui::LayoutContext, _app: &AppContext) {
        // No additional decoration to lay out — paragraphs are laid out by the
        // rich-text pipeline via the parent element.
    }

    fn paint(&mut self, model: &RenderState, ctx: &mut RenderContext, _app: &AppContext) {
        let content = model.content();
        let blockquote = extract_block!(self.viewport_item, content, (block, BlockItem::Blockquote{paragraph_block}) => block.blockquote(paragraph_block));

        let styles = model.styles();
        let text_styling = &styles.base_text;

        // Paint background fill across the visible block bounds.
        let background_rect = self.viewport_item.visible_bounds(ctx);
        ctx.paint
            .scene
            .draw_rect_without_hit_recording(background_rect)
            .with_background(styles.blockquote_background);

        // Left accent bar: thin vertical strip flush with the block's left
        // edge, spanning the full block height (covers multi-line quotes).
        let bar_rect = warpui::geometry::rect::RectF::new(
            background_rect.origin(),
            vec2f(BLOCKQUOTE_BAR_WIDTH, background_rect.size().y()),
        );
        ctx.paint
            .scene
            .draw_rect_without_hit_recording(bar_rect)
            .with_background(styles.blockquote_bar_color);

        let content_origin = blockquote.content_origin();
        if !self.placeholder.paint(content_origin, model, ctx) {
            for paragraph in blockquote.paragraphs() {
                ctx.draw_paragraph(&paragraph, text_styling, model);
            }
        }
    }
}
