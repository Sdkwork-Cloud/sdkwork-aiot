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
    }).catch((error) => {
      wx.showToast({
        title: error instanceof Error ? error.message : '设备列表加载失败',
        icon: 'none',
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
    }).catch((error) => {
      wx.showToast({
        title: error instanceof Error ? error.message : '语音命令发送失败',
        icon: 'none',
      });
    });
  },
});
