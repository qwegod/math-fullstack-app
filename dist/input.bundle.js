/*
 * ATTENTION: The "eval" devtool has been used (maybe by default in mode: "development").
 * This devtool is neither made for production nor for readable output files.
 * It uses "eval()" calls to create a separate source file in the browser devtools.
 * If you are trying to read the output file, select a different devtool (https://webpack.js.org/configuration/devtool/)
 * or disable the default devtool with "devtool: false".
 * If you are looking for production-ready output files, see mode: "production" (https://webpack.js.org/configuration/mode/).
 */
/******/ (() => { // webpackBootstrap
/******/ 	"use strict";
/******/ 	var __webpack_modules__ = ({

/***/ "./scripts/bot.js":
/*!************************!*\
  !*** ./scripts/bot.js ***!
  \************************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export */ __webpack_require__.d(__webpack_exports__, {\n/* harmony export */   calculate: () => (/* binding */ calculate)\n/* harmony export */ });\nfunction calculate(a, b, c) {\r\n    let element_bot = document.getElementById(\"bot_text\");\r\n    let element_user = document.getElementById(\"user_text\");\r\n\r\n\r\n    element_bot.textContent = element_bot.textContent + square(a, b, c);\r\n    element_user.textContent = element_user.textContent + `${a}x² + ${b}x + ${c}c = 0`;\r\n    \r\n    document.getElementById(\"a\").value = \"\";\r\n    document.getElementById(\"b\").value = \"\";\r\n    document.getElementById(\"c\").value = \"\";\r\n}\r\n\r\n\r\n\r\nfunction square(a, b, c) {\r\n    let d = Math.pow(parseInt(b), 2) - 4 * parseInt(a) * parseInt(c);\r\n    let x1 = (-parseInt(b) + Math.sqrt(d)) / (2 * parseInt(a));\r\n    let x2 = (-parseInt(b) - Math.sqrt(d)) / (2 * parseInt(a));\r\n    return `x₁ = ${x1}, x₂ = ${x2}`;\r\n}\n\n//# sourceURL=webpack:///./scripts/bot.js?");

/***/ }),

/***/ "./scripts/input.js":
/*!**************************!*\
  !*** ./scripts/input.js ***!
  \**************************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _bot_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./bot.js */ \"./scripts/bot.js\");\n\r\n\r\nwindow.send_input = function() {\r\n    let a = document.getElementById(\"a\");\r\n    let b = document.getElementById(\"b\");\r\n    let c = document.getElementById(\"c\");\r\n    let element_bot = document.getElementById(\"bot_text\");\r\n    let element_user = document.getElementById(\"user_text\");\r\n    element_user.textContent = \"Question: \";\r\n    element_bot.textContent = \"Result: \";\r\n    (0,_bot_js__WEBPACK_IMPORTED_MODULE_0__.calculate)(a.value, b.value, c.value);\r\n}\r\n\n\n//# sourceURL=webpack:///./scripts/input.js?");

/***/ })

/******/ 	});
/************************************************************************/
/******/ 	// The module cache
/******/ 	var __webpack_module_cache__ = {};
/******/ 	
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/ 		// Check if module is in cache
/******/ 		var cachedModule = __webpack_module_cache__[moduleId];
/******/ 		if (cachedModule !== undefined) {
/******/ 			return cachedModule.exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = __webpack_module_cache__[moduleId] = {
/******/ 			// no module.id needed
/******/ 			// no module.loaded needed
/******/ 			exports: {}
/******/ 		};
/******/ 	
/******/ 		// Execute the module function
/******/ 		__webpack_modules__[moduleId](module, module.exports, __webpack_require__);
/******/ 	
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/ 	
/************************************************************************/
/******/ 	/* webpack/runtime/define property getters */
/******/ 	(() => {
/******/ 		// define getter functions for harmony exports
/******/ 		__webpack_require__.d = (exports, definition) => {
/******/ 			for(var key in definition) {
/******/ 				if(__webpack_require__.o(definition, key) && !__webpack_require__.o(exports, key)) {
/******/ 					Object.defineProperty(exports, key, { enumerable: true, get: definition[key] });
/******/ 				}
/******/ 			}
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/hasOwnProperty shorthand */
/******/ 	(() => {
/******/ 		__webpack_require__.o = (obj, prop) => (Object.prototype.hasOwnProperty.call(obj, prop))
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/make namespace object */
/******/ 	(() => {
/******/ 		// define __esModule on exports
/******/ 		__webpack_require__.r = (exports) => {
/******/ 			if(typeof Symbol !== 'undefined' && Symbol.toStringTag) {
/******/ 				Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
/******/ 			}
/******/ 			Object.defineProperty(exports, '__esModule', { value: true });
/******/ 		};
/******/ 	})();
/******/ 	
/************************************************************************/
/******/ 	
/******/ 	// startup
/******/ 	// Load entry module and return exports
/******/ 	// This entry module can't be inlined because the eval devtool is used.
/******/ 	var __webpack_exports__ = __webpack_require__("./scripts/input.js");
/******/ 	
/******/ })()
;