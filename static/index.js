"use strict";
console.log(`
⠀⠀⠀⣿⣿⡆⠀⠀⢸⣷⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⡇⠀⠀⣾⣿⡆⠀
⠀⠀⠀⣿⣿⡇⠀⠀⢸⣿⢰⣿⡆⠀⣾⣿⡆⠀⣾⣷ ⣿⣿⡇⠀⠀⣿⣿⡇⠀
⠀⠀⠀⣿⣿⡇⠀⠀⢸⣿⠘⣿⣿⣤⣿⣿⣿⣤⣿⡇⢻⣿⡇⠀⠀⣿⣿⡇⠀
⠀⠀⠀⣿⣿⡇⠀⠀⢸⡿⠀⢹⣿⣿⣿⣿⣿⣿⣿⠁⢸⣿⣇⠀⢀⣿⣿⠇⠀
⠀⠀⠀⠙⢿⣷⣶⣶⡿⠁⠀⠈⣿⣿⠟⠀⣿⣿⠇⠀⠈⠻⣿⣶⣾⡿⠋⠀⠀

Check the repo:
https://github.com/Pineapple217/meta-tune
`);
import { createApp } from "https://unpkg.com/petite-vue@0.2.2/dist/petite-vue.es.js";

createApp({
  track_identifier: "",
  track: null,

  async getTrack(e) {
    e.target.blur(); // hide keyboard for mobile
    const track_id = extractTrackId(this.track_identifier);
    if (!track_id) return;
    try {
      let respone = await fetch(`./api/track/${track_id}`);
      let track_json = await respone.json();
      this.track = track_json;
      console.log(track_json);
    } catch (err) {}
  },
  floatToPer(f) {
    let x = f * 100;
    return x.toFixed(2);
  },
}).mount();

function extractTrackId(urlOrId) {
  const urlPattern = /\/track\/([a-zA-Z0-9]+)/;

  const urlMatch = urlPattern.exec(urlOrId);
  if (urlMatch && urlMatch.length >= 2) {
    const trackId = urlMatch[1];
    if (trackId.length === 22) {
      return trackId;
    } else {
      return null;
    }
  }

  if (urlOrId.length === 22) {
    return urlOrId;
  } else {
    return null;
  }
}
