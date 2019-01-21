// This file is automatically generated.

#![allow(non_camel_case_types)]

use std::fmt;
use std::mem;

pub type xcb_bool32_t = u32;
pub type xcb_visualid_t = u32;
pub type xcb_timestamp_t = u32;
pub type xcb_keysym_t = u32;
pub type xcb_keycode_t = u8;
pub type xcb_keycode32_t = u32;
pub type xcb_button_t = u8;
pub type xcb_window_t = u32;
pub type xcb_pixmap_t = u32;
pub type xcb_cursor_t = u32;
pub type xcb_font_t = u32;
pub type xcb_gcontext_t = u32;
pub type xcb_colormap_t = u32;
pub type xcb_atom_t = u32;

#[repr(C)]
pub union xcb_drawable_t {
    pub window: xcb_window_t,
    pub pixmap: xcb_pixmap_t,
}

#[repr(C)]
pub union xcb_fontable_t {
    pub font: xcb_font_t,
    pub gcontext: xcb_gcontext_t,
}

pub type xcb_visual_class_t = u32;
pub const XCB_VISUAL_CLASS_STATIC_GRAY: xcb_visual_class_t = 0;
pub const XCB_VISUAL_CLASS_GRAY_SCALE: xcb_visual_class_t = 1;
pub const XCB_VISUAL_CLASS_STATIC_COLOR: xcb_visual_class_t = 2;
pub const XCB_VISUAL_CLASS_PSEUDO_COLOR: xcb_visual_class_t = 3;
pub const XCB_VISUAL_CLASS_TRUE_COLOR: xcb_visual_class_t = 4;
pub const XCB_VISUAL_CLASS_DIRECT_COLOR: xcb_visual_class_t = 5;

pub type xcb_event_mask_t = u32;
pub const XCB_EVENT_MASK_NO_EVENT: xcb_event_mask_t = 0;
pub const XCB_EVENT_MASK_KEY_PRESS: xcb_event_mask_t = 1 << 0;
pub const XCB_EVENT_MASK_KEY_RELEASE: xcb_event_mask_t = 1 << 1;
pub const XCB_EVENT_MASK_BUTTON_PRESS: xcb_event_mask_t = 1 << 2;
pub const XCB_EVENT_MASK_BUTTON_RELEASE: xcb_event_mask_t = 1 << 3;
pub const XCB_EVENT_MASK_ENTER_WINDOW: xcb_event_mask_t = 1 << 4;
pub const XCB_EVENT_MASK_LEAVE_WINDOW: xcb_event_mask_t = 1 << 5;
pub const XCB_EVENT_MASK_POINTER_MOTION: xcb_event_mask_t = 1 << 6;
pub const XCB_EVENT_MASK_POINTER_MOTION_HINT: xcb_event_mask_t = 1 << 7;
pub const XCB_EVENT_MASK_BUTTON1_MOTION: xcb_event_mask_t = 1 << 8;
pub const XCB_EVENT_MASK_BUTTON2_MOTION: xcb_event_mask_t = 1 << 9;
pub const XCB_EVENT_MASK_BUTTON3_MOTION: xcb_event_mask_t = 1 << 10;
pub const XCB_EVENT_MASK_BUTTON4_MOTION: xcb_event_mask_t = 1 << 11;
pub const XCB_EVENT_MASK_BUTTON5_MOTION: xcb_event_mask_t = 1 << 12;
pub const XCB_EVENT_MASK_BUTTON_MOTION: xcb_event_mask_t = 1 << 13;
pub const XCB_EVENT_MASK_KEYMAP_STATE: xcb_event_mask_t = 1 << 14;
pub const XCB_EVENT_MASK_EXPOSURE: xcb_event_mask_t = 1 << 15;
pub const XCB_EVENT_MASK_VISIBILITY_CHANGE: xcb_event_mask_t = 1 << 16;
pub const XCB_EVENT_MASK_STRUCTURE_NOTIFY: xcb_event_mask_t = 1 << 17;
pub const XCB_EVENT_MASK_RESIZE_REDIRECT: xcb_event_mask_t = 1 << 18;
pub const XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY: xcb_event_mask_t = 1 << 19;
pub const XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT: xcb_event_mask_t = 1 << 20;
pub const XCB_EVENT_MASK_FOCUS_CHANGE: xcb_event_mask_t = 1 << 21;
pub const XCB_EVENT_MASK_PROPERTY_CHANGE: xcb_event_mask_t = 1 << 22;
pub const XCB_EVENT_MASK_COLOR_MAP_CHANGE: xcb_event_mask_t = 1 << 23;
pub const XCB_EVENT_MASK_OWNER_GRAB_BUTTON: xcb_event_mask_t = 1 << 24;

pub type xcb_backing_store_t = u32;
pub const XCB_BACKING_STORE_NOT_USEFUL: xcb_backing_store_t = 0;
pub const XCB_BACKING_STORE_WHEN_MAPPED: xcb_backing_store_t = 1;
pub const XCB_BACKING_STORE_ALWAYS: xcb_backing_store_t = 2;

pub type xcb_image_order_t = u32;
pub const XCB_IMAGE_ORDER_LSB_FIRST: xcb_image_order_t = 0;
pub const XCB_IMAGE_ORDER_MSB_FIRST: xcb_image_order_t = 1;

pub type xcb_mod_mask_t = u32;
pub const XCB_MOD_MASK_SHIFT: xcb_mod_mask_t = 1 << 0;
pub const XCB_MOD_MASK_LOCK: xcb_mod_mask_t = 1 << 1;
pub const XCB_MOD_MASK_CONTROL: xcb_mod_mask_t = 1 << 2;
pub const XCB_MOD_MASK_1: xcb_mod_mask_t = 1 << 3;
pub const XCB_MOD_MASK_2: xcb_mod_mask_t = 1 << 4;
pub const XCB_MOD_MASK_3: xcb_mod_mask_t = 1 << 5;
pub const XCB_MOD_MASK_4: xcb_mod_mask_t = 1 << 6;
pub const XCB_MOD_MASK_5: xcb_mod_mask_t = 1 << 7;
pub const XCB_MOD_MASK_ANY: xcb_mod_mask_t = 1 << 15;

pub type xcb_key_but_mask_t = u32;
pub const XCB_KEY_BUT_MASK_SHIFT: xcb_key_but_mask_t = 1 << 0;
pub const XCB_KEY_BUT_MASK_LOCK: xcb_key_but_mask_t = 1 << 1;
pub const XCB_KEY_BUT_MASK_CONTROL: xcb_key_but_mask_t = 1 << 2;
pub const XCB_KEY_BUT_MASK_MOD1: xcb_key_but_mask_t = 1 << 3;
pub const XCB_KEY_BUT_MASK_MOD2: xcb_key_but_mask_t = 1 << 4;
pub const XCB_KEY_BUT_MASK_MOD3: xcb_key_but_mask_t = 1 << 5;
pub const XCB_KEY_BUT_MASK_MOD4: xcb_key_but_mask_t = 1 << 6;
pub const XCB_KEY_BUT_MASK_MOD5: xcb_key_but_mask_t = 1 << 7;
pub const XCB_KEY_BUT_MASK_BUTTON1: xcb_key_but_mask_t = 1 << 8;
pub const XCB_KEY_BUT_MASK_BUTTON2: xcb_key_but_mask_t = 1 << 9;
pub const XCB_KEY_BUT_MASK_BUTTON3: xcb_key_but_mask_t = 1 << 10;
pub const XCB_KEY_BUT_MASK_BUTTON4: xcb_key_but_mask_t = 1 << 11;
pub const XCB_KEY_BUT_MASK_BUTTON5: xcb_key_but_mask_t = 1 << 12;

pub type xcb_window_enum_t = u32;
pub const XCB_WINDOW_NONE: xcb_window_enum_t = 0;

pub type xcb_button_mask_t = u32;
pub const XCB_BUTTON_MASK_1: xcb_button_mask_t = 1 << 8;
pub const XCB_BUTTON_MASK_2: xcb_button_mask_t = 1 << 9;
pub const XCB_BUTTON_MASK_3: xcb_button_mask_t = 1 << 10;
pub const XCB_BUTTON_MASK_4: xcb_button_mask_t = 1 << 11;
pub const XCB_BUTTON_MASK_5: xcb_button_mask_t = 1 << 12;
pub const XCB_BUTTON_MASK_ANY: xcb_button_mask_t = 1 << 15;

pub type xcb_motion_t = u32;
pub const XCB_MOTION_NORMAL: xcb_motion_t = 0;
pub const XCB_MOTION_HINT: xcb_motion_t = 1;

pub type xcb_notify_detail_t = u32;
pub const XCB_NOTIFY_DETAIL_ANCESTOR: xcb_notify_detail_t = 0;
pub const XCB_NOTIFY_DETAIL_VIRTUAL: xcb_notify_detail_t = 1;
pub const XCB_NOTIFY_DETAIL_INFERIOR: xcb_notify_detail_t = 2;
pub const XCB_NOTIFY_DETAIL_NONLINEAR: xcb_notify_detail_t = 3;
pub const XCB_NOTIFY_DETAIL_NONLINEAR_VIRTUAL: xcb_notify_detail_t = 4;
pub const XCB_NOTIFY_DETAIL_POINTER: xcb_notify_detail_t = 5;
pub const XCB_NOTIFY_DETAIL_POINTER_ROOT: xcb_notify_detail_t = 6;
pub const XCB_NOTIFY_DETAIL_NONE: xcb_notify_detail_t = 7;

pub type xcb_notify_mode_t = u32;
pub const XCB_NOTIFY_MODE_NORMAL: xcb_notify_mode_t = 0;
pub const XCB_NOTIFY_MODE_GRAB: xcb_notify_mode_t = 1;
pub const XCB_NOTIFY_MODE_UNGRAB: xcb_notify_mode_t = 2;
pub const XCB_NOTIFY_MODE_WHILE_GRABBED: xcb_notify_mode_t = 3;

pub type xcb_visibility_t = u32;
pub const XCB_VISIBILITY_UNOBSCURED: xcb_visibility_t = 0;
pub const XCB_VISIBILITY_PARTIALLY_OBSCURED: xcb_visibility_t = 1;
pub const XCB_VISIBILITY_FULLY_OBSCURED: xcb_visibility_t = 2;

pub type xcb_place_t = u32;
pub const XCB_PLACE_ON_TOP: xcb_place_t = 0;
pub const XCB_PLACE_ON_BOTTOM: xcb_place_t = 1;

pub type xcb_property_t = u32;
pub const XCB_PROPERTY_NEW_VALUE: xcb_property_t = 0;
pub const XCB_PROPERTY_DELETE: xcb_property_t = 1;

pub type xcb_time_t = u32;
pub const XCB_TIME_CURRENT_TIME: xcb_time_t = 0;

pub type xcb_atom_enum_t = u32;
pub const XCB_ATOM_NONE: xcb_atom_enum_t = 0;
pub const XCB_ATOM_ANY: xcb_atom_enum_t = 0;
pub const XCB_ATOM_PRIMARY: xcb_atom_enum_t = 1;
pub const XCB_ATOM_SECONDARY: xcb_atom_enum_t = 2;
pub const XCB_ATOM_ARC: xcb_atom_enum_t = 3;
pub const XCB_ATOM_ATOM: xcb_atom_enum_t = 4;
pub const XCB_ATOM_BITMAP: xcb_atom_enum_t = 5;
pub const XCB_ATOM_CARDINAL: xcb_atom_enum_t = 6;
pub const XCB_ATOM_COLORMAP: xcb_atom_enum_t = 7;
pub const XCB_ATOM_CURSOR: xcb_atom_enum_t = 8;
pub const XCB_ATOM_CUT_BUFFER0: xcb_atom_enum_t = 9;
pub const XCB_ATOM_CUT_BUFFER1: xcb_atom_enum_t = 10;
pub const XCB_ATOM_CUT_BUFFER2: xcb_atom_enum_t = 11;
pub const XCB_ATOM_CUT_BUFFER3: xcb_atom_enum_t = 12;
pub const XCB_ATOM_CUT_BUFFER4: xcb_atom_enum_t = 13;
pub const XCB_ATOM_CUT_BUFFER5: xcb_atom_enum_t = 14;
pub const XCB_ATOM_CUT_BUFFER6: xcb_atom_enum_t = 15;
pub const XCB_ATOM_CUT_BUFFER7: xcb_atom_enum_t = 16;
pub const XCB_ATOM_DRAWABLE: xcb_atom_enum_t = 17;
pub const XCB_ATOM_FONT: xcb_atom_enum_t = 18;
pub const XCB_ATOM_INTEGER: xcb_atom_enum_t = 19;
pub const XCB_ATOM_PIXMAP: xcb_atom_enum_t = 20;
pub const XCB_ATOM_POINT: xcb_atom_enum_t = 21;
pub const XCB_ATOM_RECTANGLE: xcb_atom_enum_t = 22;
pub const XCB_ATOM_RESOURCE_MANAGER: xcb_atom_enum_t = 23;
pub const XCB_ATOM_RGB_COLOR_MAP: xcb_atom_enum_t = 24;
pub const XCB_ATOM_RGB_BEST_MAP: xcb_atom_enum_t = 25;
pub const XCB_ATOM_RGB_BLUE_MAP: xcb_atom_enum_t = 26;
pub const XCB_ATOM_RGB_DEFAULT_MAP: xcb_atom_enum_t = 27;
pub const XCB_ATOM_RGB_GRAY_MAP: xcb_atom_enum_t = 28;
pub const XCB_ATOM_RGB_GREEN_MAP: xcb_atom_enum_t = 29;
pub const XCB_ATOM_RGB_RED_MAP: xcb_atom_enum_t = 30;
pub const XCB_ATOM_STRING: xcb_atom_enum_t = 31;
pub const XCB_ATOM_VISUALID: xcb_atom_enum_t = 32;
pub const XCB_ATOM_WINDOW: xcb_atom_enum_t = 33;
pub const XCB_ATOM_WM_COMMAND: xcb_atom_enum_t = 34;
pub const XCB_ATOM_WM_HINTS: xcb_atom_enum_t = 35;
pub const XCB_ATOM_WM_CLIENT_MACHINE: xcb_atom_enum_t = 36;
pub const XCB_ATOM_WM_ICON_NAME: xcb_atom_enum_t = 37;
pub const XCB_ATOM_WM_ICON_SIZE: xcb_atom_enum_t = 38;
pub const XCB_ATOM_WM_NAME: xcb_atom_enum_t = 39;
pub const XCB_ATOM_WM_NORMAL_HINTS: xcb_atom_enum_t = 40;
pub const XCB_ATOM_WM_SIZE_HINTS: xcb_atom_enum_t = 41;
pub const XCB_ATOM_WM_ZOOM_HINTS: xcb_atom_enum_t = 42;
pub const XCB_ATOM_MIN_SPACE: xcb_atom_enum_t = 43;
pub const XCB_ATOM_NORM_SPACE: xcb_atom_enum_t = 44;
pub const XCB_ATOM_MAX_SPACE: xcb_atom_enum_t = 45;
pub const XCB_ATOM_END_SPACE: xcb_atom_enum_t = 46;
pub const XCB_ATOM_SUPERSCRIPT_X: xcb_atom_enum_t = 47;
pub const XCB_ATOM_SUPERSCRIPT_Y: xcb_atom_enum_t = 48;
pub const XCB_ATOM_SUBSCRIPT_X: xcb_atom_enum_t = 49;
pub const XCB_ATOM_SUBSCRIPT_Y: xcb_atom_enum_t = 50;
pub const XCB_ATOM_UNDERLINE_POSITION: xcb_atom_enum_t = 51;
pub const XCB_ATOM_UNDERLINE_THICKNESS: xcb_atom_enum_t = 52;
pub const XCB_ATOM_STRIKEOUT_ASCENT: xcb_atom_enum_t = 53;
pub const XCB_ATOM_STRIKEOUT_DESCENT: xcb_atom_enum_t = 54;
pub const XCB_ATOM_ITALIC_ANGLE: xcb_atom_enum_t = 55;
pub const XCB_ATOM_X_HEIGHT: xcb_atom_enum_t = 56;
pub const XCB_ATOM_QUAD_WIDTH: xcb_atom_enum_t = 57;
pub const XCB_ATOM_WEIGHT: xcb_atom_enum_t = 58;
pub const XCB_ATOM_POINT_SIZE: xcb_atom_enum_t = 59;
pub const XCB_ATOM_RESOLUTION: xcb_atom_enum_t = 60;
pub const XCB_ATOM_COPYRIGHT: xcb_atom_enum_t = 61;
pub const XCB_ATOM_NOTICE: xcb_atom_enum_t = 62;
pub const XCB_ATOM_FONT_NAME: xcb_atom_enum_t = 63;
pub const XCB_ATOM_FAMILY_NAME: xcb_atom_enum_t = 64;
pub const XCB_ATOM_FULL_NAME: xcb_atom_enum_t = 65;
pub const XCB_ATOM_CAP_HEIGHT: xcb_atom_enum_t = 66;
pub const XCB_ATOM_WM_CLASS: xcb_atom_enum_t = 67;
pub const XCB_ATOM_WM_TRANSIENT_FOR: xcb_atom_enum_t = 68;

pub type xcb_colormap_state_t = u32;
pub const XCB_COLORMAP_STATE_UNINSTALLED: xcb_colormap_state_t = 0;
pub const XCB_COLORMAP_STATE_INSTALLED: xcb_colormap_state_t = 1;

pub type xcb_colormap_enum_t = u32;
pub const XCB_COLORMAP_NONE: xcb_colormap_enum_t = 0;

pub type xcb_mapping_t = u32;
pub const XCB_MAPPING_MODIFIER: xcb_mapping_t = 0;
pub const XCB_MAPPING_KEYBOARD: xcb_mapping_t = 1;
pub const XCB_MAPPING_POINTER: xcb_mapping_t = 2;

pub type xcb_window_class_t = u32;
pub const XCB_WINDOW_CLASS_COPY_FROM_PARENT: xcb_window_class_t = 0;
pub const XCB_WINDOW_CLASS_INPUT_OUTPUT: xcb_window_class_t = 1;
pub const XCB_WINDOW_CLASS_INPUT_ONLY: xcb_window_class_t = 2;

pub type xcb_cw_t = u32;
pub const XCB_CW_BACK_PIXMAP: xcb_cw_t = 1 << 0;
pub const XCB_CW_BACK_PIXEL: xcb_cw_t = 1 << 1;
pub const XCB_CW_BORDER_PIXMAP: xcb_cw_t = 1 << 2;
pub const XCB_CW_BORDER_PIXEL: xcb_cw_t = 1 << 3;
pub const XCB_CW_BIT_GRAVITY: xcb_cw_t = 1 << 4;
pub const XCB_CW_WIN_GRAVITY: xcb_cw_t = 1 << 5;
pub const XCB_CW_BACKING_STORE: xcb_cw_t = 1 << 6;
pub const XCB_CW_BACKING_PLANES: xcb_cw_t = 1 << 7;
pub const XCB_CW_BACKING_PIXEL: xcb_cw_t = 1 << 8;
pub const XCB_CW_OVERRIDE_REDIRECT: xcb_cw_t = 1 << 9;
pub const XCB_CW_SAVE_UNDER: xcb_cw_t = 1 << 10;
pub const XCB_CW_EVENT_MASK: xcb_cw_t = 1 << 11;
pub const XCB_CW_DONT_PROPAGATE: xcb_cw_t = 1 << 12;
pub const XCB_CW_COLORMAP: xcb_cw_t = 1 << 13;
pub const XCB_CW_CURSOR: xcb_cw_t = 1 << 14;

pub type xcb_back_pixmap_t = u32;
pub const XCB_BACK_PIXMAP_NONE: xcb_back_pixmap_t = 0;
pub const XCB_BACK_PIXMAP_PARENT_RELATIVE: xcb_back_pixmap_t = 1;

pub type xcb_gravity_t = u32;
pub const XCB_GRAVITY_BIT_FORGET: xcb_gravity_t = 0;
pub const XCB_GRAVITY_WIN_UNMAP: xcb_gravity_t = 0;
pub const XCB_GRAVITY_NORTH_WEST: xcb_gravity_t = 1;
pub const XCB_GRAVITY_NORTH: xcb_gravity_t = 2;
pub const XCB_GRAVITY_NORTH_EAST: xcb_gravity_t = 3;
pub const XCB_GRAVITY_WEST: xcb_gravity_t = 4;
pub const XCB_GRAVITY_CENTER: xcb_gravity_t = 5;
pub const XCB_GRAVITY_EAST: xcb_gravity_t = 6;
pub const XCB_GRAVITY_SOUTH_WEST: xcb_gravity_t = 7;
pub const XCB_GRAVITY_SOUTH: xcb_gravity_t = 8;
pub const XCB_GRAVITY_SOUTH_EAST: xcb_gravity_t = 9;
pub const XCB_GRAVITY_STATIC: xcb_gravity_t = 10;

pub type xcb_map_state_t = u32;
pub const XCB_MAP_STATE_UNMAPPED: xcb_map_state_t = 0;
pub const XCB_MAP_STATE_UNVIEWABLE: xcb_map_state_t = 1;
pub const XCB_MAP_STATE_VIEWABLE: xcb_map_state_t = 2;

pub type xcb_set_mode_t = u32;
pub const XCB_SET_MODE_INSERT: xcb_set_mode_t = 0;
pub const XCB_SET_MODE_DELETE: xcb_set_mode_t = 1;

pub type xcb_config_window_t = u32;
pub const XCB_CONFIG_WINDOW_X: xcb_config_window_t = 1 << 0;
pub const XCB_CONFIG_WINDOW_Y: xcb_config_window_t = 1 << 1;
pub const XCB_CONFIG_WINDOW_WIDTH: xcb_config_window_t = 1 << 2;
pub const XCB_CONFIG_WINDOW_HEIGHT: xcb_config_window_t = 1 << 3;
pub const XCB_CONFIG_WINDOW_BORDER_WIDTH: xcb_config_window_t = 1 << 4;
pub const XCB_CONFIG_WINDOW_SIBLING: xcb_config_window_t = 1 << 5;
pub const XCB_CONFIG_WINDOW_STACK_MODE: xcb_config_window_t = 1 << 6;

pub type xcb_stack_mode_t = u32;
pub const XCB_STACK_MODE_ABOVE: xcb_stack_mode_t = 0;
pub const XCB_STACK_MODE_BELOW: xcb_stack_mode_t = 1;
pub const XCB_STACK_MODE_TOP_IF: xcb_stack_mode_t = 2;
pub const XCB_STACK_MODE_BOTTOM_IF: xcb_stack_mode_t = 3;
pub const XCB_STACK_MODE_OPPOSITE: xcb_stack_mode_t = 4;

pub type xcb_circulate_t = u32;
pub const XCB_CIRCULATE_RAISE_LOWEST: xcb_circulate_t = 0;
pub const XCB_CIRCULATE_LOWER_HIGHEST: xcb_circulate_t = 1;

pub type xcb_prop_mode_t = u32;
pub const XCB_PROP_MODE_REPLACE: xcb_prop_mode_t = 0;
pub const XCB_PROP_MODE_PREPEND: xcb_prop_mode_t = 1;
pub const XCB_PROP_MODE_APPEND: xcb_prop_mode_t = 2;

pub type xcb_get_property_type_t = u32;
pub const XCB_GET_PROPERTY_TYPE_ANY: xcb_get_property_type_t = 0;

pub type xcb_send_event_dest_t = u32;
pub const XCB_SEND_EVENT_DEST_POINTER_WINDOW: xcb_send_event_dest_t = 0;
pub const XCB_SEND_EVENT_DEST_ITEM_FOCUS: xcb_send_event_dest_t = 1;

pub type xcb_grab_mode_t = u32;
pub const XCB_GRAB_MODE_SYNC: xcb_grab_mode_t = 0;
pub const XCB_GRAB_MODE_ASYNC: xcb_grab_mode_t = 1;

pub type xcb_grab_status_t = u32;
pub const XCB_GRAB_STATUS_SUCCESS: xcb_grab_status_t = 0;
pub const XCB_GRAB_STATUS_ALREADY_GRABBED: xcb_grab_status_t = 1;
pub const XCB_GRAB_STATUS_INVALID_TIME: xcb_grab_status_t = 2;
pub const XCB_GRAB_STATUS_NOT_VIEWABLE: xcb_grab_status_t = 3;
pub const XCB_GRAB_STATUS_FROZEN: xcb_grab_status_t = 4;

pub type xcb_cursor_enum_t = u32;
pub const XCB_CURSOR_NONE: xcb_cursor_enum_t = 0;

pub type xcb_button_index_t = u32;
pub const XCB_BUTTON_INDEX_ANY: xcb_button_index_t = 0;
pub const XCB_BUTTON_INDEX_1: xcb_button_index_t = 1;
pub const XCB_BUTTON_INDEX_2: xcb_button_index_t = 2;
pub const XCB_BUTTON_INDEX_3: xcb_button_index_t = 3;
pub const XCB_BUTTON_INDEX_4: xcb_button_index_t = 4;
pub const XCB_BUTTON_INDEX_5: xcb_button_index_t = 5;

pub type xcb_grab_t = u32;
pub const XCB_GRAB_ANY: xcb_grab_t = 0;

pub type xcb_allow_t = u32;
pub const XCB_ALLOW_ASYNC_POINTER: xcb_allow_t = 0;
pub const XCB_ALLOW_SYNC_POINTER: xcb_allow_t = 1;
pub const XCB_ALLOW_REPLAY_POINTER: xcb_allow_t = 2;
pub const XCB_ALLOW_ASYNC_KEYBOARD: xcb_allow_t = 3;
pub const XCB_ALLOW_SYNC_KEYBOARD: xcb_allow_t = 4;
pub const XCB_ALLOW_REPLAY_KEYBOARD: xcb_allow_t = 5;
pub const XCB_ALLOW_ASYNC_BOTH: xcb_allow_t = 6;
pub const XCB_ALLOW_SYNC_BOTH: xcb_allow_t = 7;

pub type xcb_input_focus_t = u32;
pub const XCB_INPUT_FOCUS_NONE: xcb_input_focus_t = 0;
pub const XCB_INPUT_FOCUS_POINTER_ROOT: xcb_input_focus_t = 1;
pub const XCB_INPUT_FOCUS_PARENT: xcb_input_focus_t = 2;
pub const XCB_INPUT_FOCUS_FOLLOW_KEYBOARD: xcb_input_focus_t = 3;

pub type xcb_font_draw_t = u32;
pub const XCB_FONT_DRAW_LEFT_TO_RIGHT: xcb_font_draw_t = 0;
pub const XCB_FONT_DRAW_RIGHT_TO_LEFT: xcb_font_draw_t = 1;

pub type xcb_gc_t = u32;
pub const XCB_GC_FUNCTION: xcb_gc_t = 1 << 0;
pub const XCB_GC_PLANE_MASK: xcb_gc_t = 1 << 1;
pub const XCB_GC_FOREGROUND: xcb_gc_t = 1 << 2;
pub const XCB_GC_BACKGROUND: xcb_gc_t = 1 << 3;
pub const XCB_GC_LINE_WIDTH: xcb_gc_t = 1 << 4;
pub const XCB_GC_LINE_STYLE: xcb_gc_t = 1 << 5;
pub const XCB_GC_CAP_STYLE: xcb_gc_t = 1 << 6;
pub const XCB_GC_JOIN_STYLE: xcb_gc_t = 1 << 7;
pub const XCB_GC_FILL_STYLE: xcb_gc_t = 1 << 8;
pub const XCB_GC_FILL_RULE: xcb_gc_t = 1 << 9;
pub const XCB_GC_TILE: xcb_gc_t = 1 << 10;
pub const XCB_GC_STIPPLE: xcb_gc_t = 1 << 11;
pub const XCB_GC_TILE_STIPPLE_ORIGIN_X: xcb_gc_t = 1 << 12;
pub const XCB_GC_TILE_STIPPLE_ORIGIN_Y: xcb_gc_t = 1 << 13;
pub const XCB_GC_FONT: xcb_gc_t = 1 << 14;
pub const XCB_GC_SUBWINDOW_MODE: xcb_gc_t = 1 << 15;
pub const XCB_GC_GRAPHICS_EXPOSURES: xcb_gc_t = 1 << 16;
pub const XCB_GC_CLIP_ORIGIN_X: xcb_gc_t = 1 << 17;
pub const XCB_GC_CLIP_ORIGIN_Y: xcb_gc_t = 1 << 18;
pub const XCB_GC_CLIP_MASK: xcb_gc_t = 1 << 19;
pub const XCB_GC_DASH_OFFSET: xcb_gc_t = 1 << 20;
pub const XCB_GC_DASH_LIST: xcb_gc_t = 1 << 21;
pub const XCB_GC_ARC_MODE: xcb_gc_t = 1 << 22;

pub type xcb_gx_t = u32;
pub const XCB_GX_CLEAR: xcb_gx_t = 0;
pub const XCB_GX_AND: xcb_gx_t = 1;
pub const XCB_GX_AND_REVERSE: xcb_gx_t = 2;
pub const XCB_GX_COPY: xcb_gx_t = 3;
pub const XCB_GX_AND_INVERTED: xcb_gx_t = 4;
pub const XCB_GX_NOOP: xcb_gx_t = 5;
pub const XCB_GX_XOR: xcb_gx_t = 6;
pub const XCB_GX_OR: xcb_gx_t = 7;
pub const XCB_GX_NOR: xcb_gx_t = 8;
pub const XCB_GX_EQUIV: xcb_gx_t = 9;
pub const XCB_GX_INVERT: xcb_gx_t = 10;
pub const XCB_GX_OR_REVERSE: xcb_gx_t = 11;
pub const XCB_GX_COPY_INVERTED: xcb_gx_t = 12;
pub const XCB_GX_OR_INVERTED: xcb_gx_t = 13;
pub const XCB_GX_NAND: xcb_gx_t = 14;
pub const XCB_GX_SET: xcb_gx_t = 15;

pub type xcb_line_style_t = u32;
pub const XCB_LINE_STYLE_SOLID: xcb_line_style_t = 0;
pub const XCB_LINE_STYLE_ON_OFF_DASH: xcb_line_style_t = 1;
pub const XCB_LINE_STYLE_DOUBLE_DASH: xcb_line_style_t = 2;

pub type xcb_cap_style_t = u32;
pub const XCB_CAP_STYLE_NOT_LAST: xcb_cap_style_t = 0;
pub const XCB_CAP_STYLE_BUTT: xcb_cap_style_t = 1;
pub const XCB_CAP_STYLE_ROUND: xcb_cap_style_t = 2;
pub const XCB_CAP_STYLE_PROJECTING: xcb_cap_style_t = 3;

pub type xcb_join_style_t = u32;
pub const XCB_JOIN_STYLE_MITER: xcb_join_style_t = 0;
pub const XCB_JOIN_STYLE_ROUND: xcb_join_style_t = 1;
pub const XCB_JOIN_STYLE_BEVEL: xcb_join_style_t = 2;

pub type xcb_fill_style_t = u32;
pub const XCB_FILL_STYLE_SOLID: xcb_fill_style_t = 0;
pub const XCB_FILL_STYLE_TILED: xcb_fill_style_t = 1;
pub const XCB_FILL_STYLE_STIPPLED: xcb_fill_style_t = 2;
pub const XCB_FILL_STYLE_OPAQUE_STIPPLED: xcb_fill_style_t = 3;

pub type xcb_fill_rule_t = u32;
pub const XCB_FILL_RULE_EVEN_ODD: xcb_fill_rule_t = 0;
pub const XCB_FILL_RULE_WINDING: xcb_fill_rule_t = 1;

pub type xcb_subwindow_mode_t = u32;
pub const XCB_SUBWINDOW_MODE_CLIP_BY_CHILDREN: xcb_subwindow_mode_t = 0;
pub const XCB_SUBWINDOW_MODE_INCLUDE_INFERIORS: xcb_subwindow_mode_t = 1;

pub type xcb_arc_mode_t = u32;
pub const XCB_ARC_MODE_CHORD: xcb_arc_mode_t = 0;
pub const XCB_ARC_MODE_PIE_SLICE: xcb_arc_mode_t = 1;

pub type xcb_clip_ordering_t = u32;
pub const XCB_CLIP_ORDERING_UNSORTED: xcb_clip_ordering_t = 0;
pub const XCB_CLIP_ORDERING_Y_SORTED: xcb_clip_ordering_t = 1;
pub const XCB_CLIP_ORDERING_YX_SORTED: xcb_clip_ordering_t = 2;
pub const XCB_CLIP_ORDERING_YX_BANDED: xcb_clip_ordering_t = 3;

pub type xcb_coord_mode_t = u32;
pub const XCB_COORD_MODE_ORIGIN: xcb_coord_mode_t = 0;
pub const XCB_COORD_MODE_PREVIOUS: xcb_coord_mode_t = 1;

pub type xcb_poly_shape_t = u32;
pub const XCB_POLY_SHAPE_COMPLEX: xcb_poly_shape_t = 0;
pub const XCB_POLY_SHAPE_NONCONVEX: xcb_poly_shape_t = 1;
pub const XCB_POLY_SHAPE_CONVEX: xcb_poly_shape_t = 2;

pub type xcb_image_format_t = u32;
pub const XCB_IMAGE_FORMAT_XY_BITMAP: xcb_image_format_t = 0;
pub const XCB_IMAGE_FORMAT_XY_PIXMAP: xcb_image_format_t = 1;
pub const XCB_IMAGE_FORMAT_Z_PIXMAP: xcb_image_format_t = 2;

pub type xcb_colormap_alloc_t = u32;
pub const XCB_COLORMAP_ALLOC_NONE: xcb_colormap_alloc_t = 0;
pub const XCB_COLORMAP_ALLOC_ALL: xcb_colormap_alloc_t = 1;

pub type xcb_color_flag_t = u32;
pub const XCB_COLOR_FLAG_RED: xcb_color_flag_t = 1 << 0;
pub const XCB_COLOR_FLAG_GREEN: xcb_color_flag_t = 1 << 1;
pub const XCB_COLOR_FLAG_BLUE: xcb_color_flag_t = 1 << 2;

pub type xcb_pixmap_enum_t = u32;
pub const XCB_PIXMAP_NONE: xcb_pixmap_enum_t = 0;

pub type xcb_font_enum_t = u32;
pub const XCB_FONT_NONE: xcb_font_enum_t = 0;

pub type xcb_query_shape_of_t = u32;
pub const XCB_QUERY_SHAPE_OF_LARGEST_CURSOR: xcb_query_shape_of_t = 0;
pub const XCB_QUERY_SHAPE_OF_FASTEST_TILE: xcb_query_shape_of_t = 1;
pub const XCB_QUERY_SHAPE_OF_FASTEST_STIPPLE: xcb_query_shape_of_t = 2;

pub type xcb_kb_t = u32;
pub const XCB_KB_KEY_CLICK_PERCENT: xcb_kb_t = 1 << 0;
pub const XCB_KB_BELL_PERCENT: xcb_kb_t = 1 << 1;
pub const XCB_KB_BELL_PITCH: xcb_kb_t = 1 << 2;
pub const XCB_KB_BELL_DURATION: xcb_kb_t = 1 << 3;
pub const XCB_KB_LED: xcb_kb_t = 1 << 4;
pub const XCB_KB_LED_MODE: xcb_kb_t = 1 << 5;
pub const XCB_KB_KEY: xcb_kb_t = 1 << 6;
pub const XCB_KB_AUTO_REPEAT_MODE: xcb_kb_t = 1 << 7;

pub type xcb_led_mode_t = u32;
pub const XCB_LED_MODE_OFF: xcb_led_mode_t = 0;
pub const XCB_LED_MODE_ON: xcb_led_mode_t = 1;

pub type xcb_auto_repeat_mode_t = u32;
pub const XCB_AUTO_REPEAT_MODE_OFF: xcb_auto_repeat_mode_t = 0;
pub const XCB_AUTO_REPEAT_MODE_ON: xcb_auto_repeat_mode_t = 1;
pub const XCB_AUTO_REPEAT_MODE_DEFAULT: xcb_auto_repeat_mode_t = 2;

pub type xcb_blanking_t = u32;
pub const XCB_BLANKING_NOT_PREFERRED: xcb_blanking_t = 0;
pub const XCB_BLANKING_PREFERRED: xcb_blanking_t = 1;
pub const XCB_BLANKING_DEFAULT: xcb_blanking_t = 2;

pub type xcb_exposures_t = u32;
pub const XCB_EXPOSURES_NOT_ALLOWED: xcb_exposures_t = 0;
pub const XCB_EXPOSURES_ALLOWED: xcb_exposures_t = 1;
pub const XCB_EXPOSURES_DEFAULT: xcb_exposures_t = 2;

pub type xcb_host_mode_t = u32;
pub const XCB_HOST_MODE_INSERT: xcb_host_mode_t = 0;
pub const XCB_HOST_MODE_DELETE: xcb_host_mode_t = 1;

pub type xcb_family_t = u32;
pub const XCB_FAMILY_INTERNET: xcb_family_t = 0;
pub const XCB_FAMILY_DE_CNET: xcb_family_t = 1;
pub const XCB_FAMILY_CHAOS: xcb_family_t = 2;
pub const XCB_FAMILY_SERVER_INTERPRETED: xcb_family_t = 5;
pub const XCB_FAMILY_INTERNET6: xcb_family_t = 6;

pub type xcb_access_control_t = u32;
pub const XCB_ACCESS_CONTROL_DISABLE: xcb_access_control_t = 0;
pub const XCB_ACCESS_CONTROL_ENABLE: xcb_access_control_t = 1;

pub type xcb_close_down_t = u32;
pub const XCB_CLOSE_DOWN_DESTROY_ALL: xcb_close_down_t = 0;
pub const XCB_CLOSE_DOWN_RETAIN_PERMANENT: xcb_close_down_t = 1;
pub const XCB_CLOSE_DOWN_RETAIN_TEMPORARY: xcb_close_down_t = 2;

pub type xcb_kill_t = u32;
pub const XCB_KILL_ALL_TEMPORARY: xcb_kill_t = 0;

pub type xcb_screen_saver_t = u32;
pub const XCB_SCREEN_SAVER_RESET: xcb_screen_saver_t = 0;
pub const XCB_SCREEN_SAVER_ACTIVE: xcb_screen_saver_t = 1;

pub type xcb_mapping_status_t = u32;
pub const XCB_MAPPING_STATUS_SUCCESS: xcb_mapping_status_t = 0;
pub const XCB_MAPPING_STATUS_BUSY: xcb_mapping_status_t = 1;
pub const XCB_MAPPING_STATUS_FAILURE: xcb_mapping_status_t = 2;

pub type xcb_map_index_t = u32;
pub const XCB_MAP_INDEX_SHIFT: xcb_map_index_t = 0;
pub const XCB_MAP_INDEX_LOCK: xcb_map_index_t = 1;
pub const XCB_MAP_INDEX_CONTROL: xcb_map_index_t = 2;
pub const XCB_MAP_INDEX_1: xcb_map_index_t = 3;
pub const XCB_MAP_INDEX_2: xcb_map_index_t = 4;
pub const XCB_MAP_INDEX_3: xcb_map_index_t = 5;
pub const XCB_MAP_INDEX_4: xcb_map_index_t = 6;
pub const XCB_MAP_INDEX_5: xcb_map_index_t = 7;

#[derive(Debug)]
#[repr(C)]
pub struct xcb_char2b_t {
    pub byte1: u8,
    pub byte2: u8,
}

#[derive(Debug)]
#[repr(C)]
pub struct xcb_point_t {
    pub x: i16,
    pub y: i16,
}

#[derive(Debug)]
#[repr(C)]
pub struct xcb_rectangle_t {
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
}

#[derive(Debug)]
#[repr(C)]
pub struct xcb_arc_t {
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub angle1: i16,
    pub angle2: i16,
}

#[derive(Debug)]
#[repr(C)]
pub struct xcb_format_t {
    pub depth: u8,
    pub bits_per_pixel: u8,
    pub scanline_pad: u8,
    pub _pad0: [u8; 5],
}

#[derive(Debug)]
#[repr(C)]
pub struct xcb_visualtype_t {
    pub visual_id: xcb_visualid_t,
    pub class: u8,
    pub bits_per_rgb_value: u8,
    pub colormap_entries: u16,
    pub red_mask: u32,
    pub green_mask: u32,
    pub blue_mask: u32,
    pub _pad0: [u8; 4],
}

#[derive(Debug)]
#[repr(C)]
pub struct xcb_depth_t {
    pub depth: u8,
    pub _pad0: [u8; 1],
    pub visuals_len: u16,
    pub _pad1: [u8; 4],
    // list
}

#[derive(Debug)]
#[repr(C)]
pub struct xcb_screen_t {
    pub root: xcb_window_t,
    pub default_colormap: xcb_colormap_t,
    pub white_pixel: u32,
    pub black_pixel: u32,
    pub current_input_masks: u32,
    pub width_in_pixels: u16,
    pub height_in_pixels: u16,
    pub width_in_millimeters: u16,
    pub height_in_millimeters: u16,
    pub min_installed_maps: u16,
    pub max_installed_maps: u16,
    pub root_visual: xcb_visualid_t,
    pub backing_stores: u8,
    pub save_unders: u8,
    pub root_depth: u8,
    pub allowed_depths_len: u8,
    // list
}

#[derive(Debug)]
#[repr(C)]
pub struct xcb_setup_request_t {
    pub byte_order: u8,
    pub _pad0: [u8; 1],
    pub protocol_major_version: u16,
    pub protocol_minor_version: u16,
    pub authorization_protocol_name_len: u16,
    pub authorization_protocol_data_len: u16,
    pub _pad1: [u8; 2],
    // list
    // list
}

#[derive(Debug)]
#[repr(C)]
pub struct xcb_setup_failed_t {
    pub status: u8,
    pub reason_len: u8,
    pub protocol_major_version: u16,
    pub protocol_minor_version: u16,
    pub length: u16,
    // list
}

#[derive(Debug)]
#[repr(C)]
pub struct xcb_setup_authenticate_t {
    pub status: u8,
    pub _pad0: [u8; 5],
    pub length: u16,
    // list
}

#[derive(Debug)]
#[repr(C)]
pub struct xcb_setup_t {
    pub status: u8,
    pub _pad0: [u8; 1],
    pub protocol_major_version: u16,
    pub protocol_minor_version: u16,
    pub length: u16,
    pub release_number: u32,
    pub resource_id_base: u32,
    pub resource_id_mask: u32,
    pub motion_buffer_size: u32,
    pub vendor_len: u16,
    pub maximum_request_length: u16,
    pub roots_len: u8,
    pub pixmap_formats_len: u8,
    pub image_byte_order: u8,
    pub bitmap_format_bit_order: u8,
    pub bitmap_format_scanline_unit: u8,
    pub bitmap_format_scanline_pad: u8,
    pub min_keycode: xcb_keycode_t,
    pub max_keycode: xcb_keycode_t,
    pub _pad1: [u8; 4],
    // list
    // list
    // list
}

#[derive(Debug)]
#[repr(C)]
pub struct xcb_timecoord_t {
    pub time: xcb_timestamp_t,
    pub x: i16,
    pub y: i16,
}

#[derive(Debug)]
#[repr(C)]
pub struct xcb_fontprop_t {
    pub name: xcb_atom_t,
    pub value: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct xcb_charinfo_t {
    pub left_side_bearing: i16,
    pub right_side_bearing: i16,
    pub character_width: i16,
    pub ascent: i16,
    pub descent: i16,
    pub attributes: u16,
}

#[derive(Debug)]
#[repr(C)]
pub struct xcb_str_t {
    pub name_len: u8,
    // list
}

#[derive(Debug)]
#[repr(C)]
pub struct xcb_segment_t {
    pub x1: i16,
    pub y1: i16,
    pub x2: i16,
    pub y2: i16,
}

#[derive(Debug)]
#[repr(C)]
pub struct xcb_coloritem_t {
    pub pixel: u32,
    pub red: u16,
    pub green: u16,
    pub blue: u16,
    pub flags: u8,
    pub _pad0: [u8; 1],
}

#[derive(Debug)]
#[repr(C)]
pub struct xcb_rgb_t {
    pub red: u16,
    pub green: u16,
    pub blue: u16,
    pub _pad0: [u8; 2],
}

#[derive(Debug)]
#[repr(C)]
pub struct xcb_host_t {
    pub family: u8,
    pub _pad0: [u8; 1],
    pub address_len: u16,
    // list
}

