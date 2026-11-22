#include QMK_KEYBOARD_H
#include "version.h"
#define MOON_LED_LEVEL LED_LEVEL
#ifndef ZSA_SAFE_RANGE
#define ZSA_SAFE_RANGE SAFE_RANGE
#endif

enum custom_keycodes {
  RGB_SLD = ZSA_SAFE_RANGE,
  ST_MACRO_0,
  ST_MACRO_1,
  ST_MACRO_2,
  ST_MACRO_3,
  ST_MACRO_4,
  ST_MACRO_5,
  ST_MACRO_6,
  ST_MACRO_7,
  ST_MACRO_8,
  ST_MACRO_9,
};




const uint16_t PROGMEM keymaps[][MATRIX_ROWS][MATRIX_COLS] = {
  [0] = LAYOUT_moonlander(
    KC_EQUAL,       KC_1,           KC_2,           KC_3,           KC_4,           KC_5,           LGUI(KC_TAB),                                   KC_PAGE_UP,     KC_6,           KC_7,           KC_8,           KC_9,           KC_0,           KC_MINUS,
    KC_DELETE,      KC_Q,           KC_W,           KC_E,           KC_R,           KC_T,           LALT(KC_TAB),                                   KC_PGDN,        KC_Y,           KC_U,           KC_I,           KC_O,           KC_P,           KC_BSLS,
    KC_ESCAPE,      MT(MOD_LGUI, KC_A),MT(MOD_LALT, KC_S),MT(MOD_LSFT, KC_D),MT(MOD_LCTL, KC_F),LT(1, KC_G),    OSL(2),                                                                         OSL(2),         LT(1, KC_H),    MT(MOD_RCTL, KC_J),MT(MOD_RSFT, KC_K),MT(MOD_RALT, KC_L),MT(MOD_RGUI, KC_SCLN),KC_QUOTE,
    KC_HOME,        KC_Z,           KC_X,           KC_C,           KC_V,           KC_B,                                           KC_N,           KC_M,           KC_COMMA,       KC_DOT,         KC_SLASH,       KC_END,
    KC_GRAVE,       CW_TOGG,        KC_LEFT,        KC_RIGHT,       LCTL(KC_SPACE), OSL(3),                                                                                                         OSL(3),         KC_UP,          KC_DOWN,        KC_LBRC,        KC_RBRC,        KC_TRANSPARENT,
    KC_SPACE,       KC_ENTER,       OSL(1),                         OSL(2),         KC_TAB,         KC_BSPC
  ),
  [1] = LAYOUT_moonlander(
    KC_TRANSPARENT, KC_F1,          KC_F2,          KC_F3,          KC_F4,          KC_F5,          KC_TRANSPARENT,                                 KC_TRANSPARENT, KC_F6,          KC_F7,          KC_F8,          KC_F9,          KC_F10,         KC_F11,
    KC_TRANSPARENT, KC_EXLM,        KC_AT,          KC_LCBR,        KC_RCBR,        KC_PIPE,        KC_TRANSPARENT,                                 KC_TRANSPARENT, KC_UP,          KC_7,           KC_8,           KC_9,           KC_ASTR,        KC_F12,
    KC_TRANSPARENT, KC_HASH,        KC_DLR,         KC_LPRN,        KC_RPRN,        KC_TRANSPARENT, KC_TRANSPARENT,                                                                 KC_TRANSPARENT, KC_TRANSPARENT, KC_4,           KC_5,           KC_6,           KC_KP_PLUS,     KC_MINUS,
    KC_TRANSPARENT, KC_PERC,        KC_CIRC,        KC_LBRC,        KC_RBRC,        KC_TILD,                                        KC_AMPR,        KC_1,           KC_2,           KC_3,           KC_BSLS,        KC_TRANSPARENT,
    KC_TRANSPARENT, KC_COMMA,       KC_TRANSPARENT, KC_TRANSPARENT, KC_TRANSPARENT, KC_MS_BTN3,                                                                                                     RGB_TOG,        KC_TRANSPARENT, KC_DOT,         KC_0,           KC_EQUAL,       KC_TRANSPARENT,
    KC_WWW_BACK,    KC_WWW_FORWARD, TO(0),                          KC_TRANSPARENT, KC_TRANSPARENT, KC_TRANSPARENT
  ),
  [2] = LAYOUT_moonlander(
    KC_TRANSPARENT, KC_TRANSPARENT, KC_TRANSPARENT, KC_TRANSPARENT, KC_TRANSPARENT, KC_TRANSPARENT, KC_TRANSPARENT,                                 KC_TRANSPARENT, KC_TRANSPARENT, KC_TRANSPARENT, KC_TRANSPARENT, KC_TRANSPARENT, KC_TRANSPARENT, KC_TRANSPARENT,
    KC_TRANSPARENT, ST_MACRO_0,     ST_MACRO_1,     ST_MACRO_2,     ST_MACRO_3,     ST_MACRO_4,     KC_TRANSPARENT,                                 KC_TRANSPARENT, ST_MACRO_5,     ST_MACRO_6,     ST_MACRO_7,     ST_MACRO_8,     ST_MACRO_9,     KC_TRANSPARENT,
    KC_TRANSPARENT, LALT(KC_1),     LALT(KC_2),     LALT(KC_3),     LALT(KC_4),     LALT(KC_5),     TO(0),                                                                          TO(0),          LALT(KC_6),     LALT(KC_7),     LALT(KC_8),     LALT(KC_9),     LALT(KC_0),     KC_TRANSPARENT,
    KC_TRANSPARENT, LALT(LSFT(KC_1)),LALT(LSFT(KC_2)),LALT(LSFT(KC_3)),LALT(LSFT(KC_4)),LALT(LSFT(KC_5)),                                LALT(LSFT(KC_6)),LALT(LSFT(KC_7)),LALT(LSFT(KC_8)),LALT(LSFT(KC_9)),LALT(LSFT(KC_0)),KC_TRANSPARENT,
    KC_TRANSPARENT, KC_TRANSPARENT, KC_TRANSPARENT, KC_TRANSPARENT, KC_TRANSPARENT, KC_TRANSPARENT,                                                                                                 KC_TRANSPARENT, KC_AUDIO_VOL_UP,KC_AUDIO_VOL_DOWN,KC_AUDIO_MUTE,  KC_TRANSPARENT, QK_BOOT,
    KC_TRANSPARENT, KC_TRANSPARENT, KC_TRANSPARENT,                 TO(0),          KC_TRANSPARENT, KC_TRANSPARENT
  ),
  // Custom layer uses the Windows listener's Glove80 slot map.
  [3] = LAYOUT_moonlander(
    KC_TRNS, KC_TRNS, KC_TRNS, KC_TRNS, KC_TRNS, KC_TRNS, KC_TRNS, KC_TRNS, KC_TRNS, KC_TRNS, KC_TRNS, KC_TRNS, KC_TRNS, KC_TRNS,
    KC_TRNS, KC_F22, KC_F23, KC_F24, LCTL(LSFT(KC_F13)), LCTL(LSFT(KC_F14)), KC_TRNS, KC_TRNS, LCTL(LSFT(KC_F15)), LCTL(LSFT(KC_F16)), LCTL(LSFT(KC_F17)), LCTL(LSFT(KC_F18)), LCTL(LSFT(KC_F19)), KC_TRNS,
    KC_TRNS, KC_F13, KC_F14, KC_F15, KC_F16, KC_F17, KC_TRNS, KC_TRNS, KC_F18, KC_F19, KC_F20, KC_F21, KC_TRNS, KC_TRNS,
    KC_TRNS, LCTL(LSFT(KC_F20)), LCTL(LSFT(KC_F21)), LCTL(LSFT(KC_F22)), LCTL(LSFT(KC_F23)), LCTL(LSFT(KC_F24)), LCTL(LALT(KC_F13)), LCTL(LALT(KC_F14)), KC_TRNS, KC_TRNS, KC_TRNS, KC_TRNS,
    KC_TRNS, KC_TRNS, KC_TRNS, KC_TRNS, KC_TRNS, KC_TRNS, KC_TRNS, KC_TRNS, KC_TRNS, KC_TRNS, KC_TRNS, KC_TRNS,
    KC_TRNS, KC_TRNS, KC_TRNS, KC_TRNS, KC_TRNS, KC_TRNS
  ),
};





extern rgb_config_t rgb_matrix_config;

RGB hsv_to_rgb_with_value(HSV hsv) {
  RGB rgb = hsv_to_rgb( hsv );
  float f = (float)rgb_matrix_config.hsv.v / UINT8_MAX;
  return (RGB){ f * rgb.r, f * rgb.g, f * rgb.b };
}

void keyboard_post_init_user(void) {
  rgb_matrix_enable();
}

const uint8_t PROGMEM ledmap[][RGB_MATRIX_LED_COUNT][3] = {
    [0] = { {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204}, {200,218,204} },

    [1] = { {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255}, {152,255,255} },

    [2] = { {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206}, {74,255,206} },

    [3] = { {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255}, {25,255,255} },

};

void set_layer_color(int layer) {
  for (int i = 0; i < RGB_MATRIX_LED_COUNT; i++) {
    HSV hsv = {
      .h = pgm_read_byte(&ledmap[layer][i][0]),
      .s = pgm_read_byte(&ledmap[layer][i][1]),
      .v = pgm_read_byte(&ledmap[layer][i][2]),
    };
    if (!hsv.h && !hsv.s && !hsv.v) {
        rgb_matrix_set_color( i, 0, 0, 0 );
    } else {
        RGB rgb = hsv_to_rgb_with_value(hsv);
        rgb_matrix_set_color(i, rgb.r, rgb.g, rgb.b);
    }
  }
}

bool rgb_matrix_indicators_user(void) {
  if (rawhid_state.rgb_control) {
      return false;
  }
  if (!keyboard_config.disable_layer_led) {
    switch (biton32(layer_state)) {
      case 0:
        set_layer_color(0);
        break;
      case 1:
        set_layer_color(1);
        break;
      case 2:
        set_layer_color(2);
        break;
      case 3:
        set_layer_color(3);
        break;
     default:
        if (rgb_matrix_get_flags() == LED_FLAG_NONE) {
          rgb_matrix_set_color_all(0, 0, 0);
        }
    }
  } else {
    if (rgb_matrix_get_flags() == LED_FLAG_NONE) {
      rgb_matrix_set_color_all(0, 0, 0);
    }
  }

  return true;
}




bool process_record_user(uint16_t keycode, keyrecord_t *record) {
  switch (keycode) {
  case QK_MODS ... QK_MODS_MAX:
    // Mouse and consumer keys (volume, media) with modifiers work inconsistently across operating systems,
    // this makes sure that modifiers are always applied to the key that was pressed.
    if (IS_MOUSE_KEYCODE(QK_MODS_GET_BASIC_KEYCODE(keycode)) || IS_CONSUMER_KEYCODE(QK_MODS_GET_BASIC_KEYCODE(keycode))) {
      if (record->event.pressed) {
        add_mods(QK_MODS_GET_MODS(keycode));
        send_keyboard_report();
        wait_ms(2);
        register_code(QK_MODS_GET_BASIC_KEYCODE(keycode));
        return false;
      } else {
        wait_ms(2);
        del_mods(QK_MODS_GET_MODS(keycode));
      }
    }
    break;
    case ST_MACRO_0:
    if (record->event.pressed) {
      SEND_STRING(SS_LCTL(SS_TAP(X_SPACE))SS_DELAY(50)  SS_TAP(X_1));
    }
    break;
    case ST_MACRO_1:
    if (record->event.pressed) {
      SEND_STRING(SS_LCTL(SS_TAP(X_SPACE))SS_DELAY(50)  SS_TAP(X_2));
    }
    break;
    case ST_MACRO_2:
    if (record->event.pressed) {
      SEND_STRING(SS_LCTL(SS_TAP(X_SPACE))SS_DELAY(50)  SS_TAP(X_3));
    }
    break;
    case ST_MACRO_3:
    if (record->event.pressed) {
      SEND_STRING(SS_LCTL(SS_TAP(X_SPACE))SS_DELAY(50)  SS_TAP(X_4));
    }
    break;
    case ST_MACRO_4:
    if (record->event.pressed) {
      SEND_STRING(SS_LCTL(SS_TAP(X_SPACE))SS_DELAY(50)  SS_TAP(X_5));
    }
    break;
    case ST_MACRO_5:
    if (record->event.pressed) {
      SEND_STRING(SS_LCTL(SS_TAP(X_SPACE))SS_DELAY(50)  SS_TAP(X_6));
    }
    break;
    case ST_MACRO_6:
    if (record->event.pressed) {
      SEND_STRING(SS_LCTL(SS_TAP(X_SPACE))SS_DELAY(50)  SS_TAP(X_7));
    }
    break;
    case ST_MACRO_7:
    if (record->event.pressed) {
      SEND_STRING(SS_LCTL(SS_TAP(X_SPACE))SS_DELAY(50)  SS_TAP(X_8));
    }
    break;
    case ST_MACRO_8:
    if (record->event.pressed) {
      SEND_STRING(SS_LCTL(SS_TAP(X_SPACE))SS_DELAY(50)  SS_TAP(X_9));
    }
    break;
    case ST_MACRO_9:
    if (record->event.pressed) {
      SEND_STRING(SS_LCTL(SS_TAP(X_SPACE))SS_DELAY(50)  SS_TAP(X_9)SS_DELAY(50)  SS_LCTL(SS_TAP(X_SPACE))SS_DELAY(100)  SS_TAP(X_N));
    }
    break;

    case RGB_SLD:
        if (rawhid_state.rgb_control) {
            return false;
        }
        if (record->event.pressed) {
            rgblight_mode(1);
        }
        return false;
  }
  return true;
}
