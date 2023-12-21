// ==UserScript==
// @name         Magician
// @namespace    http://tampermonkey.net/
// @version      0.1
// @description  try to take over the world!
// @author       You
// @match        https://www.nationstates.net/*
// @match        file:///C:/*/issues.html
// @match        file:///C:/*/packs.html
// @icon         https://www.google.com/s2/favicons?sz=64&domain=nationstates.net
// @grant        none
// ==/UserScript==

(function() {
    'use strict';

    // Create key handler
    document.addEventListener('keyup', function (event) { // keyup may seem less intuitive but it's already the standard in breeze-like scripts and it prevents holding down a key spamming the server
	    if (event.shiftKey || event.ctrlKey || event.altKey || document.activeElement.tagName == 'INPUT' || document.activeElement.tagName == 'TEXTAREA') { // locks you out of the script while you're holding down a modifier key or typing in an input
            return;
	    } else {
		    // Wait a second, this is inside an event listener... this is async already! We can just await it!
		    switch (event.code) { // event.code is the key that was pressed
			    case 'KeyI':
                    if (window.location.href.includes("issues.html") || window.location.href.includes("packs.html")) {
                        document.getElementsByTagName("a")[0].click();
                    // If we're on an ISSUE, we then click a UNIQUE LOCATION - the first "answer" button.=
                    } else if (window.location.href.includes("show_dilemma/dilemma=")) {
                        document.getElementsByClassName("button big icon approve")[0].click();

                    // Result page should take me to issues
                    } else if (window.location.href.includes("page=enact_dilemma")) {
                        window.close();
                        //document.location = "https://www.nationstates.net/page=dilemmas/template-overall=none";

                    // If we have a open cards button, click it
                    } else if (window.location.href.includes("page=deck")) {
                        let opencards = document.getElementsByName("open_loot_box");
                        if (opencards.length != 0) {
                            opencards[0].click();

                        // We are opening cards, not looking at our deck, even if we have 5 cards - close to open next pack (cuts pageloads)
                        } else if (document.getElementsByClassName("deckcard-container").length == 5 && !document.getElementById("deck-summary")) {
                            window.close();
                        // We are neither opening cards, nor do we have packs to open
                        } else {
                            window.close();
                        }

                    } else {
                        document.location = "https://www.nationstates.net/page=dilemmas/template-overall=none";
                    }

				    break;
            }
	    } // end else
    }); // end event listener
})();
