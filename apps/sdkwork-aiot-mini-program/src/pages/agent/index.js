const api = require('../../services/aiot-api');

Page({
  data: {
    devices: [],
    draft: '',
    selectedDeviceId: '',
    messages: [],
    sessionId: '',
    isSending: false,
  },
  onShow() {
    api.listDevices().then((response) => {
      const devices = Array.isArray(response.data) ? response.data : [];
      const selectedDeviceId = devices[0] ? (devices[0].deviceId || devices[0].id) : '';
      this.setData({
        devices,
        selectedDeviceId,
        sessionId: selectedDeviceId ? `agent-${selectedDeviceId}` : '',
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
    const index = Number(event.detail.value);
    const device = this.data.devices[index];
    const selectedDeviceId = device ? (device.deviceId || device.id) : '';
    this.setData({
      selectedDeviceId,
      sessionId: selectedDeviceId ? `agent-${selectedDeviceId}` : '',
    });
  },
  onSend() {
    const text = this.data.draft.trim();
    if (!text || !this.data.selectedDeviceId || this.data.isSending) {
      return;
    }

    const userMessage = {
      id: `user-${Date.now()}`,
      role: 'user',
      content: text,
    };
    const nextMessages = [...this.data.messages, userMessage];
    this.setData({ messages: nextMessages, draft: '', isSending: true });

    api.sendAgentChat(this.data.selectedDeviceId, text, this.data.sessionId)
      .then((response) => {
        const result = response.data?.resultPayload || response.data;
        const assistantText = typeof result === 'object' && result
          ? (result.text || result.reply || result.message || '智能体已响应')
          : '智能体已响应';
        this.setData({
          messages: [
            ...nextMessages,
            {
              id: `assistant-${Date.now()}`,
              role: 'assistant',
              content: String(assistantText),
            },
          ],
          isSending: false,
        });
      })
      .catch((error) => {
        wx.showToast({
          title: error instanceof Error ? error.message : '智能体消息发送失败',
          icon: 'none',
        });
        this.setData({ isSending: false });
      });
  },
});
