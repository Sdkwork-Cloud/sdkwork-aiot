const api = require('../../services/aiot-api');

Page({
  data: { nodes: [] },
  onShow() {
    api.listDevices().then((response) => {
      this.setData({ nodes: Array.isArray(response.data) ? response.data : [] });
    }).catch(() => this.setData({ nodes: [] }));
  },
});
