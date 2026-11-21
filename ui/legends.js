const labels = {
  BSLH: '\\', BSPC: 'Backspace', COMMA: ',', DEL: 'Delete', DOT: '.', DOWN: 'Down arrow', END: 'End',
  EQUAL: '=', ESC: 'Esc', FSLH: '/', GRAVE: 'Grave accent', LBKT: '[', RBKT: ']', LEFT: 'Left arrow',
  RIGHT: 'Right arrow', UP: 'Up arrow', MINUS: '-', PG_DN: 'Page Down', PG_UP: 'Page Up', RET: 'Enter',
  SEMI: ';', SPACE: 'Space', SQT: "'", TAB: 'Tab', CAPS: 'Caps Lock', INS: 'Insert', KP_DOT: 'Numpad .',
  KP_ENTER: 'Numpad Enter', KP_MINUS: 'Numpad -', KP_PLUS: 'Numpad +', KP_NUM: 'Num Lock',
  PAUSE_BREAK: 'Pause / Break', PSCRN: 'Print Screen',
  C_BRI_DN: 'Screen dimmer', C_BRI_UP: 'Screen brighter', C_MUTE: 'Mute audio', C_NEXT: 'Next track',
  C_PP: 'Play / Pause', C_PREV: 'Previous track', C_VOL_DN: 'Volume down', C_VOL_UP: 'Volume up',
  LGUI: 'Left Windows', RGUI: 'Right Windows', LALT: 'Left Alt', RALT: 'Right Alt',
  LSHFT: 'Left Shift', RSHFT: 'Right Shift', LCTRL: 'Left Ctrl', RCTRL: 'Right Ctrl',
  bootloader: 'Enter bootloader', reset: 'Restart keyboard', 'bt BT_CLR': 'Clear Bluetooth profile',
  'bt BT_CLR_ALL': 'Clear all Bluetooth profiles', bt_0: 'Bluetooth profile 1', bt_1: 'Bluetooth profile 2',
  bt_2: 'Bluetooth profile 3', bt_3: 'Bluetooth profile 4', 'out OUT_USB': 'Use USB output',
  'rgb_ug RGB_BRD': 'RGB dimmer', 'rgb_ug RGB_BRI': 'RGB brighter', 'rgb_ug RGB_EFF': 'Next RGB effect',
  'rgb_ug RGB_HUD': 'Decrease RGB hue', 'rgb_ug RGB_HUI': 'Increase RGB hue',
  'rgb_ug RGB_SAD': 'Decrease RGB saturation', 'rgb_ug RGB_SAI': 'Increase RGB saturation',
  'rgb_ug RGB_SPD': 'Slower RGB effect', 'rgb_ug RGB_SPI': 'Faster RGB effect', 'rgb_ug RGB_TOG': 'Toggle RGB',
  'LS(BSLH)': '|', 'LS(GRAVE)': '~', 'LS(LBKT)': '{', 'LS(N0)': ')', 'LS(N3)': '#', 'LS(N5)': '%',
  'LS(N9)': '(', 'LS(RBKT)': '}', '–': 'Disabled'
};
function readable(raw) {
  if (Object.hasOwn(labels, raw)) return labels[raw];
  const dual = raw.match(/^(.+) \/ ([LR][A-Z]+|1)$/);
  if (dual) return readable(dual[1]) + ' · Hold ' + (dual[2] === '1' ? 'Special layer' : readable(dual[2]));
  const modified = raw.match(/^(LC|LS|LA)\((.+)\)$/);
  if (modified) return {LC:'Ctrl',LS:'Shift',LA:'Alt'}[modified[1]] + ' + ' + readable(modified[2]);
  return raw.replace(/^N([0-9])$/, '$1').replace(/^Num ([0-9])$/, 'Numpad $1');
}

// Thumb centers and angles follow MoErgo's published editor diagram.
// Firmware indices retain the binding order of the original layout.
const thumbs = {
  52:[1184,1149,20], 53:[1314,1207,30], 54:[1434,1295,45],
  55:[1629,1293,-45], 56:[1749,1204,-30], 57:[1880,1147,-20],
  69:[1052,1273,15], 70:[1191,1330,25], 71:[1326,1425,45],
  72:[1728,1432,-45], 73:[1859,1334,-25], 74:[1999,1275,-15]
};
