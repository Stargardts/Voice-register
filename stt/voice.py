import speech_recognition as sr

r = sr.Recognizer()
recording = sr.AudioFile('recording.wav')

with recording as source:
    audio = r.record(source)

result = r.recognize_google(audio)
print(result)
