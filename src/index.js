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
  // exposed to all expressions
  count: 0,
  track_id: "",
  name: "",
  // getters
  get plusOne() {
    return this.count + 1;
  },

  async onEnter() {
    this.count++;
    console.log(this.track_id);
    let x = await fetch(`./api/track/${this.track_id}`);
    let y = await x.json();
    this.name = y.name;
  },
}).mount();

// const main = () => {};
// addEventListener("DOMContentLoaded", main);
