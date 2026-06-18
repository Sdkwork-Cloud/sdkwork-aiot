const DEFAULT_APP_API_BASE_URL = 'http://127.0.0.1:8082';

function getAppApiBaseUrl() {
  return wx.getStorageSync('SDKWORK_AIOT_APP_API_BASE_URL') || DEFAULT_APP_API_BASE_URL;
}

function request(path, options = {}) {
  return new Promise((resolve, reject) => {
    wx.request({
      url: `${getAppApiBaseUrl()}/app/v3/api${path}`,
      method: options.method || 'GET',
      data: options.data,
      header: {
        'Authorization': `Bearer ${wx.getStorageSync('SDKWORK_AUTH_TOKEN') || ''}`,
        'Access-Token': wx.getStorageSync('SDKWORK_ACCESS_TOKEN') || '',
        'Content-Type': 'application/json',
        ...(options.header || {}),
      },
      success: (response) => resolve(response.data),
      fail: reject,
    });
  });
}

module.exports = {
  listDevices() {
    return request('/iot/devices');
  },
  sendSpeakCommand(deviceId, text) {
    return request(`/iot/devices/${deviceId}/commands`, {
      method: 'POST',
      data: {
        capabilityName: 'audio.playback',
        commandName: 'speak',
        payload: { text, lang: 'zh-CN' },
      },
    });
  },
  sendAgentChat(deviceId, text, sessionId) {
    return request(`/iot/devices/${deviceId}/commands`, {
      method: 'POST',
      data: {
        capabilityName: 'assistant',
        commandName: 'chat',
        payload: { text, lang: 'zh-CN' },
        sessionId,
      },
    });
  },
};
