"use strict";
console.log(`
⠀⠀⠀⣿⣿⡆⠀⠀⢸⣷⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⡇⠀⠀⣾⣿⡆⠀
⠀⠀⠀⣿⣿⡇⠀⠀⢸⣿⢰⣿⡆⠀⣾⣿⡆⠀⣾⣷ ⣿⣿⡇⠀⠀⣿⣿⡇⠀
⠀⠀⠀⣿⣿⡇⠀⠀⢸⣿⠘⣿⣿⣤⣿⣿⣿⣤⣿⡇⢻⣿⡇⠀⠀⣿⣿⡇⠀
⠀⠀⠀⣿⣿⡇⠀⠀⢸⡿⠀⢹⣿⣿⣿⣿⣿⣿⣿⠁⢸⣿⣇⠀⢀⣿⣿⠇⠀
⠀⠀⠀⠙⢿⣷⣶⣶⡿⠁⠀⠈⣿⣿⠟⠀⣿⣿⠇⠀⠈⠻⣿⣶⣾⡿⠋⠀⠀

Check the repo:
https://github.com/Pineapple217/Muze-Site
`);
import { createApp } from "https://unpkg.com/petite-vue@0.2.2/dist/petite-vue.es.js";

createApp({
  track_identifier: "aaaaaaa",
  track: null,

  async getTrack() {
    if (!this.track_identifier) return;
    const track_id = extractTrackId(this.track_identifier);
    let respone = await fetch(`./api/track/${track_id}`);
    let track_json = await respone.json();
    this.track = track_json;
  },
}).mount();

function extractTrackId(urlOrId) {
  const urlPattern = /\/track\/([a-zA-Z0-9]+)/;
  const urlMatch = urlPattern.exec(urlOrId);
  if (urlMatch && urlMatch.length >= 2) {
    return urlMatch[1];
  }
  return urlOrId;
}
