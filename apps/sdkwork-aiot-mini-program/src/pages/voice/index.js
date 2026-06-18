const api = require('../../services/aiot-api');

Page({
  data: {
    devices: [],
    draft: '',
    selectedDeviceId: '',
  },
  onShow() {
    api.listDevices().then((response) => {
      const devices = Array.isArray(response.data) ? response.data : [];
      this.setData({
        devices,
        selectedDeviceId: devices[0] ? (devices[0].deviceId || devices[0].id) : '',
      });
    });
  },
  onInput(event) {
    this.setData({ draft: event.detail.value });
  },
  onDeviceChange(event) {
    this.setData({ selectedDeviceId: event.detail.value });
  },
  onSpeak() {
    const text = this.data.draft.trim();
    if (!text || !this.data.selectedDeviceId) return;
    api.sendSpeakCommand(this.data.selectedDeviceId, text).then(() => {
      wx.showToast({ title: '已发送语音命令', icon: 'success' });
      this.setData({ draft: '' });
    });
  },
});
