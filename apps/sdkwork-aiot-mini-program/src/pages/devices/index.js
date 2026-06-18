const api = require('../../services/aiot-api');

Page({
  data: {
    devices: [],
  },
  onShow() {
    api.listDevices().then((response) => {
      this.setData({ devices: Array.isArray(response.data) ? response.data : [] });
    }).catch(() => {
      this.setData({ devices: [] });
    });
  },
});
