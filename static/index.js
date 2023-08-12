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

const app = createApp({
  track_identifier: "",
  loading: false,
  track: null,

  async getTrack(e) {
    if (e) e.target.blur(); // hide keyboard for mobile
    const track_id = extractTrackId(this.track_identifier);
    if (!track_id) return;
    try {
      this.loading = true;
      let respone = await fetch(`./api/track/${track_id}`);
      let track_json = await respone.json();
      this.loading = false;
      this.track = track_json;
      console.log(track_json);
      const url = new URL(window.location.href);
      url.searchParams.set("track_id", track_id);
      window.history.pushState({}, "", url.toString());
    } catch (err) {}
  },
  intToPitch(integer) {
    const pitchClassSymbols = [
      "C",
      "C♯/D♭",
      "D",
      "D♯/E♭",
      "E",
      "F",
      "F♯/G♭",
      "G",
      "G♯/A♭",
      "A",
      "A♯/B♭",
      "B",
    ];

    if (integer >= 0 && integer <= 11) {
      return pitchClassSymbols[integer];
    } else {
      return "Invalid pitch class";
    }
  },
  mounted() {
    const url = new URL(window.location.href);
    const track_id = url.searchParams.get("track_id");
    this.track_identifier = track_id;
    this.getTrack();
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
