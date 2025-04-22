import React, { useState } from 'react';
import ClimbingSessionForm from './ClimbingSessionForm';
import ClimbMetricsForm from './ClimbMetricsForm';
import WorkoutSessionForm from './WorkoutSessionForm';
// import ViewLogs from './ViewLogs';

const TabbedLogger: React.FC = () => {
  const [activeTab, setActiveTab] = useState<'climb' | 'metrics' | 'workout' | 'logs'>('climb');

  const tabClass = (tab: string) =>
    `px-4 py-2 rounded-t-lg text-sm sm:text-base font-medium transition-colors duration-200 ${
      activeTab === tab
        ? 'bg-blue-600 text-white'
        : 'bg-gray-100 text-gray-600 hover:bg-gray-200'
    }`;

  return (
    <div className="max-w-4xl mx-auto p-4 sm:p-6 md:p-10 text-gray-800">
      <div className="flex space-x-2 sm:space-x-4 border-b">
        <button className={tabClass('climb')} onClick={() => setActiveTab('climb')}>
          Climb Session
        </button>
        <button className={tabClass('metrics')} onClick={() => setActiveTab('metrics')}>
          Climb Metrics
        </button>
        <button className={tabClass('workout')} onClick={() => setActiveTab('workout')}>
          Workout Session
        </button>
        <button className={tabClass('logs')} onClick={() => setActiveTab('logs')}>
          View Logs
        </button>
      </div>

      <div className="mt-8">
        {activeTab === 'climb' && <ClimbingSessionForm />}
        {activeTab === 'metrics' && <ClimbMetricsForm />}
        {activeTab === 'workout' && <WorkoutSessionForm />}
        {/* {activeTab === 'logs' && <ViewLogs />} */}
      </div>
    </div>
  );
};

export default TabbedLogger;
