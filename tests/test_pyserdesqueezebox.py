# -*- coding: utf-8 -*-
import os

import pytest
import pyserdesqueezebox



current_dir = os.path.abspath(os.path.dirname(__file__))
path = os.path.join(current_dir, "zen-of-python.txt")


@pytest.fixture(scope="session", autouse=True)
def textfile():
    test_json = """[
    {
        "name": "Salle chacha",
        "uuid": "********************************",
        "id": "**:**:**:**:**:**",
        "ip": "192.168.*.*:*****",
        "model": "Squeezebox Touch",
        "firmware_version": "7.8.0-r16754",
        "signal_strength": 88,
        "mixer": {
            "volume": "42",
            "bass": "50",
            "treble": "50",
            "power": "on"
        },
        "play_state": "pause",
        "song_currently_played": {
            "index_in_playlist" : 3,
            "seconds_played": 183.890504037857,
            "duration": "258.466",
            "artist": "The Smashing Pumpkins",
            "album": "Mellon Collie and the Infinite Sadness (2012 - Remaster)",
            "title": "Bullet With Butterfly Wings",
            "is_remote": true,
            "path": "spotify://track:4qMzPtAZe0C9KWpWIzvZAP"
        }
    },
    {
    "name": "Musique salle de bain",
    "uuid": "********************************",
    "id": "**:**:**:**:**:**",
    "ip": "192.168.*.*:*****",
    "model": "Squeezebox Radio",
    "firmware_version": "7.7.3-r16676",
    "signal_strength": 88,
    "mixer": {
        "volume": "42",
        "bass": "50",
        "treble": "50",
        "power": "on"
    },
    "play_state": "pause",
    "song_currently_played": {
        "index_in_playlist" : 3,
        "seconds_played": 183.890504037857,
        "duration": "258.466",
        "artist": "The Smashing Pumpkins",
        "album": "Mellon Collie and the Infinite Sadness (2012 - Remaster)",
        "title": "Bullet With Butterfly Wings",
        "is_remote": true,
        "path": "spotify://track:4qMzPtAZe0C9KWpWIzvZAP"
    }
}
]"""

    with open(path, "w") as f:
        f.write(test_json)
    yield
    os.remove(path)







def test_strong_typed_parse():
    json_text = ""
    with open(path, "r") as f:
        json_text = f.read()
    stl = pyserdesqueezebox.json2PlayerList(json_text)
    assert (len(stl) == 2)
    assert (stl[0].firmware_version == "7.8.0-r16754")
