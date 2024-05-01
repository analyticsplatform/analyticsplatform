"use client"

// app/menu.tsx

import React, { useState, useEffect, useRef } from 'react';
import { usePathname } from 'next/navigation'
import { Sidebar } from 'flowbite-react';
import DashboardIcon from '@mui/icons-material/Dashboard';
import DataUsageIcon from '@mui/icons-material/DataUsage';
import MapIcon from '@mui/icons-material/Map';

type CustomFlowbiteTheme = {
  sidebar: {
    root: {
      inner: string;
    };
  };
};

const sidebarTheme: CustomFlowbiteTheme['sidebar'] = {
  root: {
    inner: 'h-full overflow-y-auto overflow-x-hidden py-6 px-4 bg-gray-800'
  }
}

const createSidebarItems = (pages: string[], currentPath: string) => {
  const itemClassNames = "flex items-center px-4 py-2 text-base font-normal text-gray-300 rounded-lg hover:bg-gray-700 hover:text-white";
  const currentItemClassNames = "flex items-center px-4 py-2 text-base font-medium text-white bg-gray-900 hover:bg-gray-900 rounded-lg";
  
  return pages.map(page => {
    const pageName = page.charAt(0).toUpperCase() + page.slice(1);
    let icon;

    switch (page) {
      case 'dashboard':
        icon = <DashboardIcon className="w-6 h-6 mr-4" />;
        break;
      case 'data':
        icon = <DataUsageIcon className="w-6 h-6 mr-4" />;
        break;
      case 'map':
        icon = <MapIcon className="w-6 h-6 mr-4" />;
        break;
      default:
        icon = null;
    }

    const href = page === "dashboard" ? "/" : `/${page}`;
    const isCurrentPage = currentPath === href;

    return (
      <Sidebar.Item 
        href={href} 
        key={page} 
        className={isCurrentPage ? currentItemClassNames : itemClassNames}
      >
        {icon}
        <span>{pageName}</span>
      </Sidebar.Item>
    );
  });
};

const MySidebar = () => {
  const pathname = usePathname()
  const [isOpen, setIsOpen] = useState(false); 
  const sidebarRef = useRef<HTMLDivElement | null>(null);

  const toggleSidebar = () => {
    if (window.innerWidth < 768) {
      setIsOpen(!isOpen);
    }
  };

  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (window.innerWidth < 768 && sidebarRef.current && !sidebarRef.current.contains(event.target as Node)) {
        setIsOpen(false);
      }
    };

    document.addEventListener("mousedown", handleClickOutside);
    return () => {
      document.removeEventListener("mousedown", handleClickOutside);
    };
  }, []);

  useEffect(() => {
    const handleResize = () => {
      setIsOpen(window.innerWidth >= 768);
    };

    handleResize();
    window.addEventListener('resize', handleResize);
    return () => window.removeEventListener('resize', handleResize);
  }, []);

  return (
    <>
      <button
        className={`fixed top-0 left-0 z-40 p-4 md:hidden text-gray-300 hover:text-white focus:outline-none ${isOpen ? 'invisible' : 'block'}`}
        onClick={toggleSidebar}
        aria-label="Toggle sidebar"
      >
        <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 6h16M4 12h16m-7 6h7" />
        </svg>
      </button>

      <div
        ref={sidebarRef}
        className={`fixed inset-y-0 left-0 transform border-r border-gray-700 sb:translate-x-0 ${isOpen ? 'translate-x-0' : '-translate-x-full'} sb:transition-none transition-transform duration-200 ease-in-out z-30`}
        style={{ width: '256px' }}
      >
        <Sidebar theme={sidebarTheme}>
          <Sidebar.Logo href="#" img="" className="px-4 py-6 text-white text-2xl font-semibold">
            Analytics Platform
          </Sidebar.Logo>
          <Sidebar.Items className="mt-8">
            <Sidebar.ItemGroup>
              {createSidebarItems(["dashboard", "data", "map"], pathname)}
            </Sidebar.ItemGroup>
          </Sidebar.Items>
        </Sidebar>
      </div>
    </>
  );
};

export default MySidebar;
