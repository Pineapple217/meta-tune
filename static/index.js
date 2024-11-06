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

let root = document.body;
let loading = false;
let track = null;
let input = "";

let progressBar = {
  view: (vnode) => {
    return m(
      "div",
      {
        class: "progress-wrap progress",
        title: `${vnode.attrs.progress}/100`,
      },
      m("div", {
        class: "progress-bar progress",
        style: `left: -${100 - vnode.attrs.progress}%;`,
      })
    );
  },
};

m.mount(root, {
  oninit: async () => {
    const url = new URL(window.location.href);
    const track_id = url.searchParams.get("track_id");
    if (track_id == null) return;
    await getTrack(track_id);
    input = track_id;
  },
  view: () => {
    return m("div", { id: "main" }, [
      m("hgroup", { id: "header" }, [
        m("h1", "Meta-Tune"),
        m("h2", "Getting all the metadata"),
      ]),
      m("div", { id: "search" }, [
        m("input", {
          type: "search",
          placeholder: "Enter id or url",
          oncreate: (e) => {
            e.dom.focus();
          },
          oninput: async (e) => {
            input = e.target.value;
            let x = await getTrack(input);
            if (x) e.target.blur();
          },
          value: input,
        }),
        loading
          ? m("div", { class: "lds-dual-ring" })
          : track != null
          ? m("div", { id: "track-info" }, [
              m("hgroup", [
                m("div", [
                  m("div", { class: "cover-art" }, [
                    m(
                      "a",
                      {
                        target: "_blank",
                        href: track.images[0].url,
                      },
                      m("img", {
                        alt: "cover art",
                        src: track.images[1].url,
                        width: 300,
                        height: 300,
                      })
                    ),
                  ]),
                  m(
                    "audio",
                    {
                      controls: true,
                      preload: "auto",
                      controlslist: "noplaybackrate nodownload",
                      volume: "0.3",
                    },
                    m("source", { src: track.preview_url, type: "audio/mpeg" })
                  ),
                ]),
                m("div", { id: "titles" }, [
                  m("h1", [
                    m("a", { href: track.url, target: "_blank" }, track.name),
                    track.explicit
                      ? m(
                          "span",
                          {
                            class: "material-symbols-outlined explicit",
                            title: "Explicit",
                          },
                          "explicit"
                        )
                      : null,
                  ]),
                  m("h3", track.artists.join(", ")),
                ]),
              ]),
              m("div", { id: "info-list" }, [
                m("p", "Duration"),
                m("p", formatSeconds(track.duration)),
                track.release_date ? m("p", "Release date") : null,
                track.release_date ? m("p", track.release_date) : null,
                m("p", "Popularity"),
                m(progressBar, { progress: track.popularity }),
                track.genres.length > 0
                  ? ("p", "Genre" + (track.genres.length > 1 ? "s" : ""))
                  : null,
                track.genres.length > 0
                  ? m("p", track.genres.join(", "))
                  : null,
                m("p", m.trust("&nbsp")),
                m("p", m.trust("&nbsp")),
                m("p", "Acousticness"),
                m(progressBar, {
                  progress: floatToPer(track.audio_features.acousticness),
                }),
                m("p", "Danceability"),
                m(progressBar, {
                  progress: floatToPer(track.audio_features.danceability),
                }),
                m("p", "Energy"),
                m(progressBar, {
                  progress: floatToPer(track.audio_features.energy),
                }),
                m("p", "Instrumentalness"),
                m(progressBar, {
                  progress: floatToPer(track.audio_features.instrumentalness),
                }),
                m("p", "Liveness"),
                m(progressBar, {
                  progress: floatToPer(track.audio_features.liveness),
                }),
                m("p", "Speechiness"),
                m(progressBar, {
                  progress: floatToPer(track.audio_features.speechiness),
                }),
                m("p", "Valence"),
                m(progressBar, {
                  progress: floatToPer(track.audio_features.valence),
                }),
                m("p", m.trust("&nbsp")),
                m("p", m.trust("&nbsp")),
                m("p", "Loudness"),
                m("p", track.audio_features.loudness, " dB"),
                m("p", "Tempo"),
                m("p", Math.round(track.audio_features.tempo), " BPM"),
                track.audio_features.key !== -1 ? m("p", "Key") : null,
                track.audio_features.key !== -1
                  ? m("p", intToPitch(track.audio_features.key))
                  : null,
                m("p", "Mode"),
                m("p", track.audio_features.mode == 1 ? "Major" : "Minor"),
                m("p", "Time signature"),
                m("p", track.audio_features.time_signature, "/4"),
              ]),
            ])
          : null,
      ]),
    ]);
  },
});

async function getTrack(track_identifier) {
  const link_id = extractSpotifylinkId(track_identifier);
  if (link_id) {
    loading = true;
    try {
      let respone = await fetch(`./api/spotifylink/${link_id}`);
      let url = await respone.json();
      track_identifier = url;
    } catch (err) {}
  }
  const track_id = extractTrackId(track_identifier);
  if (track && track_id == track.id) return false;
  if (!track_id) return false;
  try {
    loading = true;
    track = await m.request({
      method: "GET",
      url: "/api/track/:id",
      params: { id: track_id },
    });
    track["id"] = track_id;
    loading = false;
    const url = new URL(window.location.href);
    url.searchParams.set("track_id", track_id);
    window.history.pushState({}, "", url.toString());
  } catch (err) {
    console.log(err);
  }
  return true;
}

function formatSeconds(seconds) {
  const minutes = Math.floor(seconds / 60);
  const remainingSeconds = seconds % 60;
  const formattedSeconds = String(remainingSeconds).padStart(2, "0");
  return `${minutes}:${formattedSeconds}`;
}

function floatToPer(f) {
  let x = f * 100;
  return x.toFixed(2);
}

function intToPitch(integer) {
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
}

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

function extractSpotifylinkId(url) {
  const regex = /^https:\/\/spotify\.link\/([A-Za-z0-9]+)$/;
  const match = url.match(regex);

  if (match && match[1]) {
    return match[1];
  } else {
    return null;
  }
}
