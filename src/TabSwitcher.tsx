import { useState, type FC } from 'react';
import { invoke } from '@tauri-apps/api/core';

type TabLabel = 'essay-app' | 'perplexity-ai';

const TabSwitcher: FC = () => {
  const [activeTab, setActiveTab] = useState<TabLabel>('essay-app');

  const handleTabClick = (tab: TabLabel) => {
    if (tab === activeTab) return;

    // Hide the currently active window
    invoke('hide_window', { label: activeTab }).catch((error: unknown) => {
      console.info('Warning: Failed to hide current window', error);
    });

    // Show the new window and update the active tab state
    invoke('show_window', { label: tab })
      .then(() => {
        setActiveTab(tab);
      })
      .catch((error: unknown) => {
        console.info('Warning: Failed to show window', error);
      });
  };

  return (
    <div>
      <div style={{ display: 'flex', borderBottom: '1px solid #ccc' }}>
        <div
          style={{
            padding: '10px 20px',
            cursor: 'pointer',
            backgroundColor: activeTab === 'essay-app' ? '#ddd' : '#fff'
          }}
          onClick={() => handleTabClick('essay-app')}
        >
          Essay App
        </div>
        <div
          style={{
            padding: '10px 20px',
            cursor: 'pointer',
            backgroundColor: activeTab === 'perplexity-ai' ? '#ddd' : '#fff'
          }}
          onClick={() => handleTabClick('perplexity-ai')}
        >
          Perplexity AI
        </div>
      </div>
      <div style={{ padding: '20px' }}>
        <p>
          Currently active tab: <strong>{activeTab}</strong>
        </p>
      </div>
    </div>
  );
};

export default TabSwitcher;
