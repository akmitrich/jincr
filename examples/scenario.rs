use jincr::op;
use serde_json::json;

fn main() {
    let start = json!({
        "common": {
            "tts": false,
            "asr": false
        },
        "meta": {
            "type": "scenario",
            "name": "Тестовый сценарий",
            "version": "1.0.0"
        },
        "scenarios": {
            "answered": {
                "items": {
                    "1": {
                        "id": 1,
                        "type": "Start",
                        "icon": "flag",
                        "data": {
                            "name": "Старт",
                            "comment": "",
                            "undeletable": true,
                            "settings": {}
                        },
                        "x": 194,
                        "y": 85
                    }
                },
                "connections": {},
                "meta": {
                    "id": "answered",
                    "name": "@Ответ (основной)",
                    "type": "subscenario",
                    "version": "1.0.0"
                }
            }
        }
    });
    let say = json!({
      "data": {
        "comment": "",
        "name": "Если есть увер. > set вернуть 1 ответ с наибольшей уверен.",
        "settings": {
          "audio_record_id": null,
          "message_params": {
            "ssml": false,
            "text": "%response.nlu_output.0.answer%"
          },
          "say_type": "tts",
          "tts_params": {
            // "language": "ru-RU",
            // "provider": "tinkoff",
            // "voice": {
            //   "emotion": "neutral",
            //   "id": "alyona",
            //   "pitch": null,
            //   "rate": null,
            //   "speed": 1
            // }
          }
        }
      },
      "icon": "chat-dots",
      "id": 2,
      "type": "Say",
      "x": -681.400432900432,
      "y": 451.5178508455791
    });
    let ops = [
        op::Kind::Snap.builder().value(start),
        op::Kind::Add
            .builder()
            .path("scenarios.answered.items.2")
            .value(say),
    ];
    let doc = jincr::op::document(ops.into_iter().map(op::Builder::build));
    std::fs::write("../sc.json", serde_json::to_string_pretty(&doc).unwrap()).unwrap();
}
