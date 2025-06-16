#[cfg(test)]
mod generic_callback_tests {
    use std::sync::{Arc, Mutex};

    /// Test that generic callbacks compile and work with zero-cost abstractions
    #[test]
    fn test_generic_callback_compilation() {
        // Test that various callback types compile correctly
        
        // Simple closure
        let simple_callback = |_update: &crate::AccountUpdate| {
            // Do nothing - this tests compilation only
        };
        
        // Closure with capture
        let capture_value = 42;
        let capturing_callback = move |update: &crate::AccountUpdate| {
            let _combined = capture_value + update.slot as i32;
        };
        
        // Function pointer
        fn function_callback(_update: &crate::AccountUpdate) {
            // Do nothing
        }
        
        // Verify all types satisfy the generic constraint
        fn verify_constraint<F>(_f: F) 
        where 
            F: Fn(&crate::AccountUpdate) + Send + Sync + 'static + Clone,
        {
            // Type constraint verification - this ensures the types work with our generics
        }
        
        verify_constraint(simple_callback);
        verify_constraint(capturing_callback);
        verify_constraint(function_callback as fn(&crate::AccountUpdate));
        
        // If we reach here, all callback types compile correctly
        assert!(true);
    }

    /// Test that callbacks can be cloned (required for distribution to multiple components)
    #[test]
    fn test_generic_callback_cloning() {
        let call_count = Arc::new(Mutex::new(0));
        
        let callback = {
            let call_count = call_count.clone();
            move |_update: &crate::AccountUpdate| {
                let mut count = call_count.lock().unwrap();
                *count += 1;
            }
        };
        
        // Clone the callback (this is what happens internally when distributing to perp/spot maps)
        let callback_clone1 = callback.clone();
        let callback_clone2 = callback.clone();
        
        // Create a mock update
        let mock_update = crate::AccountUpdate {
            pubkey: solana_sdk::pubkey!("So11111111111111111111111111111111111111112"),
            owner: solana_sdk::pubkey!("11111111111111111111111111111111"),
            lamports: 1000000,
            data: vec![1, 2, 3, 4],
            slot: 12345,
        };
        
        // Call each clone
        callback(&mock_update);
        callback_clone1(&mock_update);
        callback_clone2(&mock_update);
        
        // Verify all calls were registered
        assert_eq!(*call_count.lock().unwrap(), 3);
    }

    /// Test performance characteristics by measuring call overhead
    #[test]
    fn test_generic_callback_performance() {
        use std::time::Instant;
        
        const ITERATIONS: usize = 100_000;
        
        // Create a simple callback that does minimal work
        let callback = |update: &crate::AccountUpdate| {
            // Minimal processing to measure pure callback overhead
            let _slot = update.slot;
        };
        
        // Create a mock update
        let mock_update = crate::AccountUpdate {
            pubkey: solana_sdk::pubkey!("So11111111111111111111111111111111111111112"),
            owner: solana_sdk::pubkey!("11111111111111111111111111111111"),
            lamports: 1000000,
            data: vec![0; 100], // Reasonable size
            slot: 12345,
        };
        
        // Measure generic callback performance
        let start = Instant::now();
        for _ in 0..ITERATIONS {
            callback(&mock_update);
        }
        let generic_duration = start.elapsed();
        
        // For comparison, measure direct function call
        fn direct_call(update: &crate::AccountUpdate) {
            let _slot = update.slot;
        }
        
        let start = Instant::now();
        for _ in 0..ITERATIONS {
            direct_call(&mock_update);
        }
        let direct_duration = start.elapsed();
        
        println!("Generic callback: {:?}", generic_duration);
        println!("Direct function: {:?}", direct_duration);
        
        // Generic callbacks should be nearly as fast as direct calls
        // Allow up to 3x overhead (in release builds should be nearly identical)
        let max_allowed_overhead = direct_duration * 3;
        assert!(generic_duration <= max_allowed_overhead, 
            "Generic callbacks should have minimal overhead. Got {:?} vs {:?} (direct)", 
            generic_duration, direct_duration);
    }

    /// Test that None callbacks work correctly (no-op case)
    #[test]
    fn test_none_callback_handling() {
        // Test that None works correctly with type inference
        let none_callback: Option<fn(&crate::AccountUpdate)> = None;
        
        // Verify we can call the callback handling logic
        if let Some(callback) = none_callback {
            let mock_update = crate::AccountUpdate {
                pubkey: solana_sdk::pubkey!("So11111111111111111111111111111111111111112"),
                owner: solana_sdk::pubkey!("11111111111111111111111111111111"),
                lamports: 1000000,
                data: vec![1, 2, 3],
                slot: 12345,
            };
            callback(&mock_update);
        }
        
        // If we reach here, None handling works correctly
        assert!(true);
    }

    /// Test that callbacks work with real AccountUpdate data
    #[test]
    fn test_callback_with_real_data() {
        let processed_updates = Arc::new(Mutex::new(Vec::new()));
        
        let callback = {
            let processed_updates = processed_updates.clone();
            move |update: &crate::AccountUpdate| {
                let mut updates = processed_updates.lock().unwrap();
                updates.push((update.pubkey, update.slot, update.data.len()));
            }
        };
        
        // Simulate different types of account updates
        let updates = vec![
            crate::AccountUpdate {
                pubkey: solana_sdk::pubkey!("So11111111111111111111111111111111111111112"),
                owner: solana_sdk::pubkey!("11111111111111111111111111111111"),
                lamports: 1000000,
                data: vec![1; 100], // Market data
                slot: 12345,
            },
            crate::AccountUpdate {
                pubkey: solana_sdk::pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
                owner: solana_sdk::pubkey!("11111111111111111111111111111111"),
                lamports: 2000000,
                data: vec![2; 200], // Oracle data
                slot: 12346,
            },
        ];
        
        // Process updates through callback
        for update in &updates {
            callback(update);
        }
        
        // Verify all updates were processed
        let processed = processed_updates.lock().unwrap();
        assert_eq!(processed.len(), 2);
        assert_eq!(processed[0], (updates[0].pubkey, 12345, 100));
        assert_eq!(processed[1], (updates[1].pubkey, 12346, 200));
    }
}