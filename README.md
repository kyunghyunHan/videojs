# videojs


## WebRtc
- [videojs-webrtc-plugin](https://github.com/ant-media/videojs-webrtc-plugin)
- install
```
npm install --save @antmedia/videojs-webrtc-plugin

```
```js
player.src({
  src: 'ws://localhost:5080/LiveApp/stream1.webrtc',
  iceServers: '[ { "urls": "stun:stun1.l.google.com:19302" } ]'
});
```