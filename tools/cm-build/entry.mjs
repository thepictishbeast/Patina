// Offline CodeMirror 6 bundle for Tempered Studio — exposes a small window.CM API.
import {EditorView, keymap, lineNumbers, highlightActiveLineGutter, highlightActiveLine, drawSelection, dropCursor, rectangularSelection, crosshairCursor} from "@codemirror/view";
import {EditorState, Compartment} from "@codemirror/state";
import {defaultKeymap, history, historyKeymap, indentWithTab} from "@codemirror/commands";
import {rust} from "@codemirror/lang-rust";
import {syntaxHighlighting, HighlightStyle, indentOnInput, bracketMatching, foldGutter, indentUnit} from "@codemirror/language";
import {tags as t} from "@lezer/highlight";
import {closeBrackets, closeBracketsKeymap} from "@codemirror/autocomplete";
window.CM = {
  EditorView, EditorState, Compartment, keymap, lineNumbers, highlightActiveLine, highlightActiveLineGutter,
  drawSelection, dropCursor, rectangularSelection, crosshairCursor,
  defaultKeymap, history, historyKeymap, indentWithTab,
  rust, syntaxHighlighting, HighlightStyle, indentOnInput, bracketMatching, foldGutter, indentUnit,
  closeBrackets, closeBracketsKeymap, tags: t,
};
